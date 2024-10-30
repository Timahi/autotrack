<script lang="ts" setup>
definePageMeta({
  middleware: ['profile-middleware'],
  layout: 'profile-layout',
})

const { profile } = storeToRefs(selectedProfile())
const { $vehicleService } = useNuxtApp()

const { data: vehicles, error } = useAsyncData(`profile-${profile.value.id}-vehicles`, () =>
  $vehicleService.getAll(profile.value.id)
)

console.log(vehicles, error)

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
]

const rows = computed(() =>
  (vehicles.value || []).map((vehicle) => ({
    brand: vehicle.brand,
    model: vehicle.model,
    registration: vehicle.registration,
    description: vehicle.description,
  }))
)
</script>

<template>
  <UCard
    class="h-[calc(100vh-9rem)] mt-10 mx-10 flex flex-col"
    :ui="{ body: { base: 'flex-1 overflow-y-scroll' } }"
  >
    <template #header>
      <h1 class="text-2xl font-semibold text-center">Liste des véhicules</h1>
    </template>

    <UTable
      :rows
      :columns
    >
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
        <UButton>
          <template #leading>
            <IPlusSquare />
            Nouveau véhicule
          </template>
        </UButton>
      </div>
    </template>
  </UCard>
</template>
