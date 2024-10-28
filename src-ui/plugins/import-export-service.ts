import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

export default defineNuxtPlugin(() => {
  const importExportService = {
    async import() {
      return new Promise<void>(async (resolve, reject) => {
        try {
          const path = await open({
            multiple: false,
            directory: false,
            filters: [
              {
                name: 'Sqlite Database',
                extensions: ['db', 'sqlite'],
              },
            ],
          })

          if (!path) {
            reject('Opération annulée')
          }

          await invoke<void>('import_database_to_path_command', { path })
          resolve()
        } catch (error) {
          reject(error)
        }
      })
    },

    async export() {
      return new Promise<void>(async (resolve, reject) => {
        try {
          const path = await save({
            filters: [
              {
                name: 'Sqlite Database',
                extensions: ['db', 'sqlite'],
              },
            ],
          })

          if (!path) {
            reject('Opération annulée')
          }

          await invoke<void>('export_database_to_path_command', { path })
          resolve()
        } catch (error) {
          reject(error)
        }
      })
    },
  }

  return {
    provide: {
      importExportService,
    },
  }
})
