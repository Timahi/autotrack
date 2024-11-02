<script lang="ts" setup>
const profile = await useProfile()
const { $vehicleService } = useNuxtApp()

const { data: vehicles } = useAsyncData(`profile-${profile.value.id}-vehicles`, () =>
  $vehicleService.getAll(profile.value.id)
)

const columns = [
  {
    key: 'brand',
    label: 'Marque',
    sortable: true,
  },
  {
    key: 'model',
    label: 'Modèle',
    sortable: true,
  },
  {
    key: 'registration',
    label: 'Immatriculation',
    sortable: true,
  },
  {
    key: 'description',
    label: 'Description',
  },
  {
    key: 'actions',
  },
]

const rows = computed(() =>
  (vehicles.value || []).map((vehicle) => ({
    id: vehicle.id,
    brand: vehicle.brand,
    model: vehicle.model,
    registration: vehicle.registration,
    description:
      vehicle.description && vehicle.description.length > 75
        ? vehicle.description.substring(0, 75) + '…'
        : vehicle.description,
  }))
)
</script>

<template>
  <ProfileLayout>
    <UCard
      class="h-[calc(100vh-9rem)] mt-10 mx-10 flex flex-col"
      :ui="{ body: { base: 'flex-1 overflow-y-scroll' } }"
    >
      <template #header>
        <h1 class="text-2xl font-semibold text-center">Liste des véhicules</h1>
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
                :to="`/profiles/${profile.id}/vehicles/${row.id}`"
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
                :to="`/profiles/${profile.id}/vehicles/${row.id}/edit`"
                block
              >
                <IPencil class="size-6" />
              </UButton>
            </UTooltip>
          </div>
        </template>

        <template #empty-state>
          <div class="flex flex-col items-center justify-center py-10 gap-5">
            <p>Vous n'avez aucun véhicule.</p>
            <!--suppress HtmlUnknownTarget -->
            <UButton
              variant="link"
              :to="`/profiles/${profile.id}/vehicles/create`"
            >
              Créer un véhicule
            </UButton>
          </div>
        </template>
      </UTable>

      <template #footer>
        <div class="flex justify-end items-center">
          <UButton :to="`/profiles/${profile.id}/vehicles/create`">
            <template #leading>
              <IPlusSquare />
              Nouveau véhicule
            </template>
          </UButton>
        </div>
      </template>
    </UCard>
  </ProfileLayout>
</template>
