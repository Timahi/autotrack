<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

const profile = await useProfile()
const vehicle = await useVehicle()

const { $maintenanceService } = useNuxtApp()
const maintenance = await useMaintenance()

const performedAtDate = computed(() => formatDate(maintenance.value.performedAt))
const updatedAtDate = computed(() => formatDate(maintenance.value.updatedAt))
const updatedAtTime = computed(() => formatTime(maintenance.value.updatedAt))

const schema = $maintenanceService.schemas.update

type Values = z.infer<typeof schema>

const state = computed(() =>
  reactive<Values>({
    type: undefined,
    description: maintenance.value.description || undefined,
    odometer: undefined,
    performedAt: maintenance.value.performedAt,
  })
)

watch(state, (input) => {
  for (const [key, value] of Object.entries(input)) {
    if (key === 'performedAt') continue

    if (!value) {
      // @ts-ignore
      state[key] = undefined
    }
  }
})

const noChanges = computed(() => {
  for (const [key, value] of Object.entries(state)) {
    if (value) {
      switch (key) {
        case 'description':
          return value === maintenance.value.description
        case 'performedAt':
          return value === maintenance.value.performedAt
        default:
          return false
      }
    }
  }

  return true
})

const toast = useToast()
const loading = ref(false)

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $maintenanceService.update(maintenance.value.id, event.data)
    toast.add({
      id: 'update-maintenance-success',
      title: "L'entretien a été mis à jour",
      color: 'green',
    })
    await refreshNuxtData(['maintenance', `maintenance-${maintenance.value.id}`])
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'update-maintenance-error', title: error })
    } else {
      toast.add({ id: 'update-maintenance-error', title: 'Une erreur est survenue' })
    }
  } finally {
    state.value.type = undefined
    state.value.odometer = undefined
    loading.value = false
  }
}

const deleteModalOpen = ref(false)
const deleteLoading = ref(false)

async function handleDelete() {
  deleteLoading.value = true

  try {
    await $maintenanceService.delete(maintenance.value.id)
    toast.add({
      id: 'maintenance-delete-success',
      title: "L'entretien a été supprimé",
      color: 'green',
    })
    await refreshNuxtData(['maintenance', `maintenance-${maintenance.value.id}`])
    await navigateTo(`/profiles/${profile.value.id}/vehicles/${vehicle.value.id}`)
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'maintenance-delete-error', title: error })
    } else {
      toast.add({ id: 'maintenance-delete-error', title: 'Une erreur est survenue' })
    }
  } finally {
    deleteLoading.value = false
  }
}
</script>

<template>
  <ProfileLayout>
    <VehicleLayout>
      <div class="h-[calc(100vh-9rem-48px)] flex flex-col items-center justify-center">
        <UCard class="w-full max-w-xl">
          <template #header>
            <h1 class="text-2xl font-semibold text-center">
              Modifier «&nbsp;{{ maintenance.type }}&nbsp;» du {{ performedAtDate }}
            </h1>
          </template>

          <UForm
            :schema="schema"
            :state="state"
            @submit="handleSubmit"
            class="space-y-4"
          >
            <UFormGroup
              label="Nature de l'entretien"
              name="type"
            >
              <div class="flex items-center gap-2">
                <UInput
                  v-model="state.type"
                  :placeholder="maintenance.type"
                  class="flex-1"
                />
                <UTooltip
                  :popper="{ arrow: true }"
                  text="Réinitialiser"
                >
                  <!--suppress PointlessBooleanExpressionJS -->
                  <UButton
                    color="white"
                    class="size-8"
                    @click="state.type = undefined"
                    :disabled="!state.type"
                  >
                    <IRotateCcw />
                  </UButton>
                </UTooltip>
              </div>
            </UFormGroup>
            <UFormGroup
              label="Description"
              name="description"
            >
              <div class="flex items-center gap-2">
                <UTextarea
                  v-model="state.description"
                  :placeholder="maintenance.description || ''"
                  class="flex-1"
                />
                <UTooltip
                  :popper="{ arrow: true }"
                  text="Réinitialiser"
                >
                  <!--suppress PointlessBooleanExpressionJS -->
                  <UButton
                    color="white"
                    class="size-8"
                    @click="state.description = maintenance.description || undefined"
                    :disabled="state.description === maintenance.description"
                  >
                    <IRotateCcw />
                  </UButton>
                </UTooltip>
              </div>
            </UFormGroup>
            <UFormGroup
              label="Relevé kilométrique"
              name="odometer"
            >
              <div class="flex items-center gap-2">
                <UInput
                  v-model="state.odometer"
                  class="flex-1"
                  type="number"
                  min="1"
                  :placeholder="String(maintenance.odometer)"
                />
                <UTooltip
                  :popper="{ arrow: true }"
                  text="Réinitialiser"
                >
                  <!--suppress PointlessBooleanExpressionJS -->
                  <UButton
                    color="white"
                    class="size-8"
                    @click="state.odometer = undefined"
                    :disabled="!state.odometer"
                  >
                    <IRotateCcw />
                  </UButton>
                </UTooltip>
              </div>
            </UFormGroup>

            <UFormGroup
              label="Réalisé le"
              name="performedAt"
            >
              <div class="flex items-center gap-2">
                <UPopover
                  :popper="{ placement: 'top' }"
                  class="flex-1"
                >
                  <UButton
                    icon="i-lucide-calendar-days"
                    :label="formatFullDate(state.performedAt)"
                    color="white"
                    block
                  />

                  <template #panel="{ close }">
                    <DatePicker
                      v-model="state.performedAt"
                      @close="close"
                    />
                  </template>
                </UPopover>

                <UTooltip
                  :popper="{ arrow: true }"
                  text="Réinitialiser"
                >
                  <!--suppress PointlessBooleanExpressionJS -->
                  <UButton
                    color="white"
                    class="size-8"
                    @click="state.performedAt = maintenance.performedAt"
                    :disabled="state.performedAt === maintenance.performedAt"
                  >
                    <IRotateCcw />
                  </UButton>
                </UTooltip>
              </div>
            </UFormGroup>

            <UButton
              type="submit"
              :loading="loading"
              :disabled="noChanges"
              block
            >
              Mettre à jour
            </UButton>
          </UForm>
          <template #footer>
            <div class="flex items-center justify-between">
              <p class="text-sm text-neutral-500">
                Dernière modification le {{ updatedAtDate }} à {{ updatedAtTime }}
              </p>
              <UTooltip
                text="Supprimer l'entretien"
                :popper="{ arrow: true }"
              >
                <UButton
                  variant="soft"
                  size="sm"
                  square
                  @click="deleteModalOpen = true"
                >
                  <ITrash2 class="size-5" />
                </UButton>
              </UTooltip>
            </div>
          </template>
        </UCard>

        <UModal v-model="deleteModalOpen">
          <UCard>
            <div class="space-y-4">
              <h3 class="font-semibold text-xl text-center text-balance truncate">
                Êtes-vous sûr de vouloir supprimer «&nbsp;{{ maintenance.type }}&nbsp;» du
                {{ performedAtDate }} ?
              </h3>

              <!--              <p class="text-neutral-400 text-sm text-pretty">-->
              <!--                Toutes les réparations associées au véhicule «&nbsp;{{ vehicle.brand }}-->
              <!--                {{ vehicle.model }}&nbsp;» immatriculé-->
              <!--                <span class="font-semibold">{{ vehicle.registration }}</span-->
              <!--                >, si elles n'ont pas été exportées, seront définitivement perdues.-->
              <!--              </p>-->

              <div class="grid grid-cols-2 gap-4">
                <UButton
                  variant="soft"
                  @click="deleteModalOpen = false"
                  block
                >
                  Annuler
                </UButton>
                <UButton
                  :loading="deleteLoading"
                  @click="
                    () => {
                      deleteModalOpen = false
                      handleDelete()
                    }
                  "
                  block
                >
                  Confirmer
                </UButton>
              </div>
            </div>
          </UCard>
        </UModal>
      </div>
    </VehicleLayout>
  </ProfileLayout>
</template>
