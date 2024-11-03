<script lang="ts" setup>
const profile = await useProfile()
const vehicle = await useVehicle()
const { $maintenanceService } = useNuxtApp()

const { data: maintenances } = useAsyncData(`vehicle-${vehicle.value.id}-maintenance`, () =>
  $maintenanceService.getAll(vehicle.value.id)
)

const columns = [
  {
    key: 'type',
    label: 'Nature',
    sortable: true,
  },
  {
    key: 'description',
    label: 'Description',
  },
  {
    key: 'odometer',
    label: 'Relevé kilométrique',
    sortable: true,
  },
  {
    key: 'performedAt',
    label: 'Réalisé le',
    sortable: true,
  },
  {
    key: 'actions',
  },
]

const rows = computed(() =>
  (maintenances.value || []).map((maintenance) => ({
    id: maintenance.id,
    type: maintenance.type,
    description:
      maintenance.description && maintenance.description.length > 75
        ? maintenance.description.substring(0, 75) + '…'
        : maintenance.description,
    odometer: formatNumber(maintenance.odometer),
    performedAt: formatDate(maintenance.performedAt),
  }))
)
</script>

<template>
  <ProfileLayout>
    <VehicleLayout>
      <UCard
        class="h-[calc(100vh-9rem-48px)] mt-10 mx-10 flex flex-col"
        :ui="{ body: { base: 'flex-1 overflow-y-scroll' } }"
      >
        <template #header>
          <h1 class="text-2xl font-semibold text-center">Liste des entretiens</h1>
        </template>

        <UTable
          :rows="rows"
          :columns="columns"
        >
          <template #actions-data="{ row }">
            <div class="flex items-center justify-end gap-4">
              <UTooltip
                :popper="{ arrow: true }"
                text="Ouvrir"
              >
                <UButton
                  size="sm"
                  color="white"
                  class="size-10"
                  :to="`/profiles/${profile.id}/vehicles/${vehicle.id}/maintenance/${row.id}`"
                  block
                >
                  <IExternalLink class="size-6" />
                </UButton>
              </UTooltip>
              <UTooltip
                :popper="{ arrow: true }"
                text="Modifier"
              >
                <UButton
                  size="sm"
                  color="white"
                  class="size-10"
                  :to="`/profiles/${profile.id}/vehicles/${vehicle.id}/maintenance/${row.id}/edit`"
                  block
                >
                  <IPencil class="size-6" />
                </UButton>
              </UTooltip>
            </div>
          </template>

          <template #empty-state>
            <div class="flex flex-col items-center justify-center py-10 gap-5">
              <p>Aucun entretien n'a été enregistré pour ce véhicule.</p>
              <!--suppress HtmlUnknownTarget -->
              <UButton
                variant="link"
                :to="`/profiles/${profile.id}/vehicles/${vehicle.id}/maintenance/create`"
              >
                Ajouter un entretien
              </UButton>
            </div>
          </template>
        </UTable>

        <template #footer>
          <div class="flex justify-end items-center">
            <UButton :to="`/profiles/${profile.id}/vehicles/${vehicle.id}/maintenance/create`">
              <template #leading>
                <IPlusSquare />
                Nouvel entretien
              </template>
            </UButton>
          </div>
        </template>
      </UCard>
    </VehicleLayout>
  </ProfileLayout>
</template>
