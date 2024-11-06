<script lang="ts" setup>
import type { FormSubmitEvent, HorizontalNavigationLink } from '#ui/types'
import { z } from 'zod'

const profile = await useProfile()
const vehicle = await useVehicle()

const odometerUpdatedAtDate = computed(() => formatDate(vehicle.value.odometerUpdatedAt))
const odometerUpdatedAtTime = computed(() => formatTime(vehicle.value.odometerUpdatedAt))

const route = useRoute()
const { $maintenanceBookletService, $vehicleService } = useNuxtApp()

const odometerShouldBeUpdated = computed(() => {
  const diff = 7 * 24 * 60 * 60 * 1000 // 7 days
  return Date.now() - vehicle.value.odometerUpdatedAt.getTime() > diff
})

const odometerModalOpen = ref(false)

const maintenanceBookletLoading = ref(false)

const toast = useToast()

async function handleCreateMaintenanceBooklet() {
  maintenanceBookletLoading.value = true
  try {
    await $maintenanceBookletService.generate(vehicle.value.id)
    toast.add({
      id: 'booklet-creation-success',
      title: "Le carnet d'entretien a été généré",
      color: 'green',
    })
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'booklet-creation-error', title: error })
    } else {
      toast.add({ id: 'booklet-creation-error', title: 'Une erreur est survenue' })
    }
  } finally {
    maintenanceBookletLoading.value = false
  }
}

const links: HorizontalNavigationLink[][] = [
  [
    {
      label: 'Entretien',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-maintenance',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/maintenance`
        )
      ).value,
      icon: 'i-lucide-wrench',
    },
    {
      label: 'Rappels',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-reminders',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/reminders`
        )
      ).value,
      icon: 'i-lucide-bell',
    },
    {
      label: 'Contrôle Technique',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-inspections',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/inspections`
        )
      ).value,
      icon: 'i-lucide-clipboard-list',
    },
  ],

  [
    {
      label: 'Modifier',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-edit',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      icon: 'i-lucide-file-cog',
    },
  ],
]

const schema = z.object({
  odometer: z
    .number()
    .int()
    .gte(
      vehicle.value.odometer,
      "Vous ne pouvez pas diminuer le compteur kilométrique. Si vous avez fait une erreur lors de la création du véhicule, merci de passer par l'onglet « Modifier »"
    ),
})

type Values = z.infer<typeof schema>

const state = computed(() =>
  reactive<Values>({
    odometer: vehicle.value.odometer,
  })
)

const loading = ref(false)

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $vehicleService.update(vehicle.value.id, vehicle.value.odometer, event.data)
    await refreshNuxtData(['vehicles', `vehicle-${vehicle.value.id}`])
    toast.add({
      id: 'odometer-update-success',
      title: 'Le compteur kilométrique a été mis à jour',
      color: 'green',
    })
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'odometer-update-error', title: error })
    } else {
      toast.add({ id: 'odometer-update-error', title: 'Une erreur est survenue' })
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div>
    <header class="px-10 flex items-center gap-1.5">
      <h2 class="text-nowrap text-sm mr-2 text-neutral-300">
        {{ vehicle.brand }} {{ vehicle.model }}
        <span class="text-neutral-400">- {{ vehicle.registration }}</span>
      </h2>
      <UHorizontalNavigation :links="links" />
      <UChip
        :show="odometerShouldBeUpdated"
        size="lg"
      >
        <UTooltip
          :text="
            odometerShouldBeUpdated
              ? 'Dernière mise à jour il y a plus d\'une semaine !'
              : undefined
          "
          :popper="{ arrow: true }"
        >
          <UButton
            :variant="odometerShouldBeUpdated ? 'soft' : 'ghost'"
            :color="odometerShouldBeUpdated ? 'primary' : 'gray'"
            icon="i-lucide-gauge"
            :class="odometerShouldBeUpdated || '!text-gray-400 hover:!text-white'"
            @click="odometerModalOpen = true"
            >Mettre à jour le kilométrage
          </UButton>
        </UTooltip>
      </UChip>
      <UButton
        variant="ghost"
        color="gray"
        icon="i-lucide-notebook"
        :loading="maintenanceBookletLoading"
        @click="handleCreateMaintenanceBooklet"
        class="!text-gray-400 hover:!text-white"
      >
        Générer un carnet d'entretien
      </UButton>
    </header>

    <slot></slot>

    <UModal v-model="odometerModalOpen">
      <UCard>
        <template #header>
          <h1 class="text-xl font-semibold text-center">Mise à jour du kilométrage</h1>
        </template>

        <UForm
          :schema="schema"
          :state="state"
          @submit="handleSubmit"
          class="space-y-4"
        >
          <UFormGroup
            label="Nouveau kilométrage"
            name="odometer"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.odometer"
                class="flex-1"
                type="number"
                :min="vehicle.odometer"
              />
              <UTooltip
                :popper="{ arrow: true }"
                text="Réinitialiser"
              >
                <!--suppress PointlessBooleanExpressionJS -->
                <UButton
                  color="white"
                  class="size-8"
                  @click="state.odometer = vehicle.odometer"
                  :disabled="state.odometer === vehicle.odometer"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
            <p class="text-xs text-neutral-500 mt-0.5">
              Dernière mise à jour du compteur le {{ odometerUpdatedAtDate }} à
              {{ odometerUpdatedAtTime }}
            </p>
          </UFormGroup>

          <UButton
            type="submit"
            :loading="loading"
            :disabled="state.odometer === vehicle.odometer && !odometerShouldBeUpdated"
            block
          >
            Mettre à jour
          </UButton>
        </UForm>
      </UCard>
    </UModal>
  </div>
</template>
