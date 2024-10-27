import Database from '@tauri-apps/plugin-sql'

export default defineNuxtPlugin(async () => {
  const db = await Database.load('sqlite://autotrack.db')

  return {
    provide: {
      db,
    },
  }
})
