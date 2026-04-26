const { invoke } = window.__TAURI__.core;

function monthApp() {
  return {
    loading: false,
    error: null,
    monthSchedule: [],
    monthTitle: '',
    todayNum: new Date().getDate(),

    prayers: [
      { key: 'subuh',   name: 'Subuh' },
      { key: 'dzuhur',  name: 'Dzuhur' },
      { key: 'ashar',   name: 'Ashar' },
      { key: 'maghrib', name: 'Maghrib' },
      { key: 'isya',    name: 'Isya' },
    ],

    async init() {
      const now = new Date();
      const months = ['Januari','Februari','Maret','April','Mei','Juni','Juli','Agustus','September','Oktober','November','Desember'];
      this.monthTitle = `${months[now.getMonth()]} ${now.getFullYear()}`;
      this.todayNum = now.getDate();
      await this.loadSchedule();
    },

    async loadSchedule() {
      this.loading = true;
      this.error = null;
      try {
        const settings = await invoke('get_settings');
        if (!settings.province || !settings.city) {
          this.error = 'Lokasi belum diatur. Silakan atur di jendela utama.';
          return;
        }
        const now = new Date();
        const schedule = await invoke('get_monthly_schedule', {
          province: settings.province,
          city: settings.city,
          month: now.getMonth() + 1,
          year: now.getFullYear(),
        });
        this.monthSchedule = schedule;
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
  };
}
