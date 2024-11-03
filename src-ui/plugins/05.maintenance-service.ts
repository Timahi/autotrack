import { invoke } from '@tauri-apps/api/core'
import { z } from 'zod'
import { EditMaintenance, Maintenance, NewMaintenance } from '~/models/maintenance'

export default defineNuxtPlugin(() => {
  const maintenanceService = {
    async getAll(vehicleId: number) {
      return new Promise<Maintenance[]>(async (resolve, reject) => {
        try {
          const data = await invoke<any[]>('get_maintenance_command', { vehicleId })
          resolve(
            data
              .map((m) => Maintenance.from(m))
              .sort((a, b) => a.performedAt.getTime() - b.performedAt.getTime())
          )
        } catch (error) {
          reject(error)
        }
      })
    },

    async getById(maintenanceId: number) {
      return new Promise<Maintenance>(async (resolve, reject) => {
        try {
          const data = await invoke<any>('get_maintenance_by_id_command', { maintenanceId })
          resolve(Maintenance.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async create(vehicleId: number, values: z.infer<typeof this.schemas.create>) {
      return new Promise<Maintenance>(async (resolve, reject) => {
        try {
          const newMaintenance = NewMaintenance.from({
            ...values,
            vehicleId,
            createdAt: new Date(),
            updatedAt: new Date(),
          }).toJSON()
          const data = await invoke<any>('create_maintenance_command', {
            vehicleId,
            newMaintenance,
          })
          resolve(Maintenance.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async update(maintenanceId: number, values: z.infer<typeof this.schemas.update>) {
      return new Promise<Maintenance>(async (resolve, reject) => {
        try {
          const editMaintenance = EditMaintenance.from({
            ...values,
            updatedAt: new Date(),
          }).toJSON()
          const data = await invoke<any>('update_maintenance_command', {
            maintenanceId,
            editMaintenance,
          })
          resolve(Maintenance.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async delete(maintenanceId: number) {
      return new Promise<void>(async (resolve, reject) => {
        try {
          await invoke('delete_maintenance_command', { maintenanceId })
          resolve()
        } catch (error) {
          reject(error)
        }
      })
    },

    schemas: {
      create: z.object({
        type: z
          .string()
          .min(1, "Le type d'entretien est obligatoire")
          .max(200, "Le type d'entretien ne peut excéder 200 caractères"),
        description: z.string().optional(),
        odometer: z
          .number()
          .int('Le relevé kilométrique doit être un nombre entier')
          .gte(1, 'Le compteur kilométrique ne peut pas être négatif'),
        performedAt: z.date(),
      }),

      update: z.object({
        type: z
          .string()
          .min(1, "Le type d'entretien est obligatoire")
          .max(200, "Le type d'entretien ne peut excéder 200 caractères")
          .optional(),
        description: z.string().optional(),
        odometer: z
          .number()
          .int('Le relevé kilométrique doit être un nombre entier')
          .gte(1, 'Le compteur kilométrique ne peut pas être négatif')
          .optional(),
        performedAt: z.date().optional(),
      }),
    },
  }

  return {
    provide: {
      maintenanceService,
    },
  }
})
