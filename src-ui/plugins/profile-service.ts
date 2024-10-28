import Profile from '~/models/profile'
import { invoke } from '@tauri-apps/api/core'
import { z } from 'zod'

export default defineNuxtPlugin(() => {
  const profileService = {
    async getAll() {
      return new Promise<Profile[]>(async (resolve, reject) => {
        try {
          const data = await invoke<any[]>('get_profiles_handler')
          resolve(data.map((p) => Profile.fromJSON(p)))
        } catch (error) {
          reject(error)
        }
      })
    },

    async create({ name }: z.infer<typeof this.schemas.create>) {
      return new Promise<Profile>(async (resolve, reject) => {
        try {
          const data = await invoke<any>('create_profile_handler', { name })
          resolve(Profile.fromJSON(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    schemas: {
      create: z.object({
        name: z
          .string()
          .min(1, 'Le nom du profil est obligatoire')
          .max(200, 'Le nom du profil ne peut excéder 200 caractères'),
      }),
    },
  }

  return {
    provide: {
      profileService,
    },
  }
})
