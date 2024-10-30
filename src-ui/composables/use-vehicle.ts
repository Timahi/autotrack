import type { Vehicle } from '~/models/vehicle'

// @ts-ignore
export default async function (): Promise<Ref<Vehicle>> {
  const route = useRoute()
  const { $vehicleService } = useNuxtApp()
  const toast = useToast()

  try {
    const { data: vehicle } = await useAsyncData(`vehicle-${route.params.vehicleId}`, () =>
      $vehicleService.getById(+route.params.vehicleId)
    )

    if (!vehicle.value) {
      // noinspection ExceptionCaughtLocallyJS
      throw 'Not found'
    }

    // @ts-ignore
    return vehicle
  } catch {
    toast.add({ id: 'vehicle-doesnt-exists', title: "Ce véhicule n'existe plus" })
    navigateTo(`/profiles/${route.params.profileId}/vehicles`)
  }
}
