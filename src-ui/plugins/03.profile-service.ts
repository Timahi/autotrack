import { EditProfile, NewProfile, Profile } from '~/models/profile'
import { invoke } from '@tauri-apps/api/core'
import { z } from 'zod'

export default defineNuxtPlugin(() => {
  const profileService = {
    async getAll() {
      return new Promise<Profile[]>(async (resolve, reject) => {
        try {
          const data = await invoke<any[]>('get_profiles_command')
          resolve(data.map((p) => Profile.from(p)))
        } catch (error) {
          reject(error)
        }
      })
    },

    async getById(profileId: number) {
      return new Promise<Profile>(async (resolve, reject) => {
        try {
          const data = await invoke<any>('get_profile_by_id_command', { profileId })
          resolve(Profile.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async create(values: z.infer<typeof this.schemas.create>) {
      return new Promise<Profile>(async (resolve, reject) => {
        try {
          const newProfile = NewProfile.from(values).toJSON()
          const data = await invoke<any>('create_profile_command', { newProfile })
          resolve(Profile.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async update(profileId: number, values: z.infer<typeof this.schemas.update>) {
      return new Promise<Profile>(async (resolve, reject) => {
        try {
          const editProfile = EditProfile.from(values).toJSON()
          const data = await invoke<any>('update_profile_command', { profileId, editProfile })
          resolve(Profile.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async delete(profileId: number) {
      return new Promise<void>(async (resolve, reject) => {
        try {
          await invoke('delete_profile_command', { profileId })
          resolve()
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

      update: z.object({
        name: z.string().max(200, 'Le nom du profil ne peut excéder 200 caractères').optional(),
      }),
    },
  }

  return {
    provide: {
      profileService,
    },
  }
})
