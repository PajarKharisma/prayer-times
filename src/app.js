const { invoke } = window.__TAURI__.core;

function prayApp() {
  return {
    view: 'main',
    confirmingExit: false,

    // State
    loading: false,
    error: null,
    settings: { province: null, city: null },
    todaySchedule: null,
    monthSchedule: [],
    nextPrayer: null,
    todayLabel: '',
    todayNum: new Date().getDate(),
    monthTitle: '',
    lastLoadedDate: null, // 'YYYY-MM-DD' of last schedule fetch

    prayers: [
      { key: 'subuh',   name: 'Subuh',   icon: '🌙' },
      { key: 'dzuhur',  name: 'Dzuhur',  icon: '☀️' },
      { key: 'ashar',   name: 'Ashar',   icon: '🌤' },
      { key: 'maghrib', name: 'Maghrib', icon: '🌅' },
      { key: 'isya',    name: 'Isya',    icon: '🌃' },
    ],

    // Settings state
    provinces: [],
    cities: [],
    selectedProvince: '',
    selectedCity: '',
    loadingProvinces: false,
    loadingCities: false,
    savingSettings: false,
    settingsError: null,

    async init() {
      const now = new Date();
      this.todayLabel = this.formatDate(now);
      this.todayNum = now.getDate();
      const months = ['Januari','Februari','Maret','April','Mei','Juni','Juli','Agustus','September','Oktober','November','Desember'];
      this.monthTitle = `${months[now.getMonth()]} ${now.getFullYear()}`;
      await this.loadSettings();
      if (this.settings.city) {
        await this.loadSchedule();
      }

      // Refresh whenever the window becomes visible again
      document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible') {
          this.onVisible();
        }
      });
    },

    async onVisible() {
      const now = new Date();
      const todayKey = `${now.getFullYear()}-${now.getMonth()}-${now.getDate()}`;

      // Always recompute which prayer is next
      if (this.todaySchedule) this.computeNextPrayer(now);

      if (this.lastLoadedDate === todayKey) return;

      // Day has changed — update labels
      this.todayLabel = this.formatDate(now);
      this.todayNum = now.getDate();
      const months = ['Januari','Februari','Maret','April','Mei','Juni','Juli','Agustus','September','Oktober','November','Desember'];
      this.monthTitle = `${months[now.getMonth()]} ${now.getFullYear()}`;

      if (!this.settings.city) return;

      // If month is the same, just re-pick today's entry from cached data
      const [loadedYear, loadedMonth] = (this.lastLoadedDate || '').split('-').map(Number);
      if (loadedYear === now.getFullYear() && loadedMonth === now.getMonth()) {
        const today = now.getDate();
        const entry = this.monthSchedule.find(d => d.tanggal === today || d.tanggal === String(today));
        if (entry) {
          this.todaySchedule = entry;
          this.computeNextPrayer(now);
          this.lastLoadedDate = todayKey;
        } else {
          await this.loadSchedule();
        }
      } else {
        // New month — fetch fresh schedule
        await this.loadSchedule();
      }
    },

    formatDate(date) {
      const days = ['Minggu','Senin','Selasa','Rabu','Kamis','Jumat','Sabtu'];
      const months = ['Jan','Feb','Mar','Apr','Mei','Jun','Jul','Agu','Sep','Okt','Nov','Des'];
      return `${days[date.getDay()]}, ${date.getDate()} ${months[date.getMonth()]} ${date.getFullYear()}`;
    },

    async loadSettings() {
      try {
        const s = await invoke('get_settings');
        this.settings = s;
        if (s.province) this.selectedProvince = s.province;
        if (s.city) this.selectedCity = s.city;
      } catch (e) {
        console.error('Failed to load settings:', e);
      }
    },

    async loadSchedule() {
      if (!this.settings.province || !this.settings.city) return;
      this.loading = true;
      this.error = null;
      try {
        const now = new Date();
        const schedule = await invoke('get_monthly_schedule', {
          province: this.settings.province,
          city: this.settings.city,
          month: now.getMonth() + 1,
          year: now.getFullYear(),
        });

        this.monthSchedule = schedule;

        // Find today's entry — tanggal is an integer (e.g. 26)
        const today = now.getDate();
        const entry = schedule.find(d => d.tanggal === today || d.tanggal === String(today));

        if (entry) {
          this.todaySchedule = entry;
          this.computeNextPrayer(now);
          this.lastLoadedDate = `${now.getFullYear()}-${now.getMonth()}-${now.getDate()}`;
        } else {
          this.error = 'Jadwal hari ini tidak ditemukan.';
        }

        // Scroll month list to today after render
        this.$nextTick(() => this.scrollToToday());
      } catch (e) {
        this.error = 'Gagal memuat jadwal: ' + e;
      } finally {
        this.loading = false;
      }
    },

    scrollToToday() {
      const list = this.$refs.monthList;
      if (!list) return;
      const todayRow = list.querySelector('.today-row');
      if (todayRow) todayRow.scrollIntoView({ block: 'center' });
    },

    computeNextPrayer(now) {
      const currentMinutes = now.getHours() * 60 + now.getMinutes();
      this.nextPrayer = null;
      for (const { key } of this.prayers) {
        const timeStr = this.todaySchedule?.[key];
        if (!timeStr) continue;
        const [h, m] = timeStr.split(':').map(Number);
        if (h * 60 + m > currentMinutes) { this.nextPrayer = key; break; }
      }
    },

    async onViewSettings() {
      this.settingsError = null;
      await this.loadProvinces();
      if (this.selectedProvince) await this.loadCities(this.selectedProvince);
    },

    async loadProvinces() {
      if (this.provinces.length > 0) return;
      this.loadingProvinces = true;
      try {
        const raw = await invoke('get_provinces');
        this.provinces = raw.map(p => typeof p === 'string' ? p : (p.name || p.provinsi || JSON.stringify(p)));
      } catch (e) {
        this.settingsError = 'Gagal memuat provinsi: ' + e;
      } finally {
        this.loadingProvinces = false;
      }
    },

    async onProvinceChange() {
      this.cities = [];
      this.selectedCity = '';
      if (!this.selectedProvince) return;
      await this.loadCities(this.selectedProvince);
    },

    async loadCities(province) {
      this.loadingCities = true;
      try {
        const raw = await invoke('get_cities', { province });
        this.cities = raw.map(c => typeof c === 'string' ? c : (c.name || c.kabkota || c.kota || JSON.stringify(c)));
      } catch (e) {
        this.settingsError = 'Gagal memuat kota: ' + e;
      } finally {
        this.loadingCities = false;
      }
    },

    async saveSettings() {
      this.savingSettings = true;
      this.settingsError = null;
      try {
        await invoke('save_settings', {
          province: this.selectedProvince,
          city: this.selectedCity,
        });
        this.settings = { province: this.selectedProvince, city: this.selectedCity };
        this.view = 'main';
        await this.loadSchedule();
      } catch (e) {
        this.settingsError = 'Gagal menyimpan: ' + e;
      } finally {
        this.savingSettings = false;
      }
    },

    async openMonthWindow() {
      try {
        await invoke('open_month_window');
      } catch (e) {
        console.error('Failed to open month window:', e);
      }
    },

    exitApp() {
      this.confirmingExit = true;
    },

    async confirmExit() {
      try {
        await invoke('exit_app');
      } catch (e) {
        console.error('Failed to exit:', e);
      }
    },
  };
}

