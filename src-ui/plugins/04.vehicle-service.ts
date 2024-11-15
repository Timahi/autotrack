import { invoke } from '@tauri-apps/api/core'
import { z } from 'zod'
import { EditVehicle, NewVehicle, Vehicle } from '~/models/vehicle'

export default defineNuxtPlugin(() => {
  const vehicleService = {
    async getAll(profileId: number) {
      return new Promise<Vehicle[]>(async (resolve, reject) => {
        try {
          const data = await invoke<any[]>('get_vehicles_command', { profileId })
          resolve(data.map((p) => Vehicle.from(p)))
        } catch (error) {
          reject(error)
        }
      })
    },

    async getById(vehicleId: number) {
      return new Promise<Vehicle>(async (resolve, reject) => {
        try {
          const data = await invoke<any>('get_vehicle_by_id_command', { vehicleId })
          resolve(Vehicle.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async create(profileId: number, values: z.infer<typeof this.schemas.create>) {
      return new Promise<Vehicle>(async (resolve, reject) => {
        try {
          const newVehicle = NewVehicle.from({
            ...values,
            profileId,
            odometerUpdatedAt: new Date(),
            createdAt: new Date(),
            updatedAt: new Date(),
          }).toJSON()
          const data = await invoke<any>('create_vehicle_command', { newVehicle })
          resolve(Vehicle.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async update(
      vehicleId: number,
      previousOdometer: number,
      values: z.infer<typeof this.schemas.update>
    ) {
      return new Promise<Vehicle>(async (resolve, reject) => {
        try {
          const odometerUpdatedAt =
            values.odometer && previousOdometer !== values.odometer ? new Date() : undefined
          const editVehicle = EditVehicle.from({
            ...values,
            odometerUpdatedAt,
            updatedAt: new Date(),
          }).toJSON()
          const data = await invoke<any>('update_vehicle_command', { vehicleId, editVehicle })
          resolve(Vehicle.from(data))
        } catch (error) {
          reject(error)
        }
      })
    },

    async delete(vehicleId: number) {
      return new Promise<void>(async (resolve, reject) => {
        try {
          await invoke('delete_vehicle_command', { vehicleId })
          resolve()
        } catch (error) {
          reject(error)
        }
      })
    },

    schemas: {
      create: z.object({
        brand: z
          .string()
          .min(1, 'La marque est obligatoire')
          .max(200, 'La marque ne peut excéder 200 caractères'),
        model: z
          .string()
          .min(1, 'Le modèle est obligatoire')
          .max(200, 'Le modèle ne peut excéder 200 caractères'),
        odometer: z
          .number()
          .int('Le compteur kilométrique doit être un nombre entier')
          .gte(1, 'Le compteur kilométrique ne peut pas être négatif'),
        registration: z
          .string()
          .min(1, "La plaque d'immatriculation est obligatoire")
          .max(200, "La plaque d'immatriculation ne peut excéder 200 caractères"),
        registrationYear: z
          .number()
          .int("Année d'immatriculation invalide")
          .gte(1900, "Année d'immatriculation invalide")
          .lte(new Date().getFullYear(), "Année d'immatriculation invalide"),
        serialNumber: z
          .string()
          .max(200, 'Le numéro de série ne peut excéder 200 caractères')
          .optional(),
        description: z
          .string()
          .max(200, 'La description ne peut excéder 200 caractères')
          .optional(),
      }),
      update: z.object({
        brand: z
          .string()
          .min(1, 'La marque est obligatoire')
          .max(200, 'La marque ne peut excéder 200 caractères')
          .optional(),
        model: z
          .string()
          .min(1, 'Le modèle est obligatoire')
          .max(200, 'Le modèle ne peut excéder 200 caractères')
          .optional(),
        odometer: z
          .number()
          .int('Le compteur kilométrique doit être un nombre entier')
          .gte(1, 'Le compteur kilométrique ne peut pas être négatif')
          .optional(),
        registration: z
          .string()
          .min(1, "La plaque d'immatriculation est obligatoire")
          .max(200, "La plaque d'immatriculation ne peut excéder 200 caractères")
          .optional(),
        registrationYear: z
          .number()
          .int("Année d'immatriculation invalide")
          .gte(1900, "Année d'immatriculation invalide")
          .lte(new Date().getFullYear(), "Année d'immatriculation invalide")
          .optional(),
        serialNumber: z
          .string()
          .max(200, 'Le numéro de série ne peut excéder 200 caractères')
          .optional(),
        description: z
          .string()
          .max(200, 'La description ne peut excéder 200 caractères')
          .optional(),
      }),
    },
  }

  return {
    provide: {
      vehicleService,
    },
  }
})
