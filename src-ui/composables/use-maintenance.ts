import type { Maintenance } from '~/models/maintenance'

// @ts-ignore
export default async function (): Promise<Ref<Maintenance>> {
  const route = useRoute()
  const { $maintenanceService } = useNuxtApp()
  const toast = useToast()

  try {
    const { data: maintenance } = await useAsyncData(
      `maintenance-${route.params.maintenanceId}`,
      () => $maintenanceService.getById(+route.params.maintenanceId)
    )

    if (!maintenance.value) {
      // noinspection ExceptionCaughtLocallyJS
      throw 'Not found'
    }

    // @ts-ignore
    return maintenance
  } catch {
    toast.add({ id: 'maintenance-doesnt-exists', title: "Cet entretien n'existe plus" })
    navigateTo(`/profiles/${route.params.profileId}/vehicles/${route.params.vehicleId}`)
  }
}
