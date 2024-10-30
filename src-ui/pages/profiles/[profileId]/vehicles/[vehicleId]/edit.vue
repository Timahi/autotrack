<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

const { $vehicleService } = useNuxtApp()
const vehicle = await useVehicle()
const profile = await useProfile()

const updatedAtDate = computed(() => formatDate(vehicle.value.updatedAt))
const updatedAtTime = computed(() => formatTime(vehicle.value.updatedAt))
const odometerUpdatedAtDate = computed(() => formatDate(vehicle.value.odometerUpdatedAt))
const odometerUpdatedAtTime = computed(() => formatTime(vehicle.value.odometerUpdatedAt))

const schema = $vehicleService.schemas.update

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  brand: undefined,
  model: undefined,
  registration: undefined,
  registrationYear: undefined,
  odometer: undefined,
  serialNumber: undefined,
  description: undefined,
})

watch(state, (input) => {
  for (const [key, value] of Object.entries(input)) {
    if (!value) {
      // @ts-ignore
      state[key] = undefined
    }
  }
})

const toast = useToast()
const loading = ref(false)

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $vehicleService.update(vehicle.value.id, vehicle.value.odometer, event.data)
    toast.add({
      id: 'update-vehicle-success',
      title: 'Le véhicule a été mis à jour',
      color: 'green',
    })
    await refreshNuxtData(['vehicles', `vehicle-${vehicle.value.id}`])
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'update-vehicle-error', title: error })
    } else {
      toast.add({ id: 'update-vehicle-error', title: 'Une erreur est survenue' })
    }
  } finally {
    for (const key of Object.keys(state)) {
      state[key as keyof typeof state] = undefined
    }
    loading.value = false
  }
}

const deleteModalOpen = ref(false)
const deleteLoading = ref(false)

async function handleDelete() {
  deleteLoading.value = true

  try {
    await $vehicleService.delete(vehicle.value.id)
    toast.add({ id: 'vehicle-delete-success', title: 'Le véhicule a été supprimé', color: 'green' })
    await refreshNuxtData(['vehicles', `vehicle-${vehicle.value.id}`])
    await navigateTo(`/profiles/${profile.value.id}/vehicles`)
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'vehicle-delete-error', title: error })
    } else {
      toast.add({ id: 'vehicle-delete-error', title: 'Une erreur est survenue' })
    }
  } finally {
    deleteLoading.value = false
  }
}
</script>

<template>
  <ProfileLayout>
    <div class="h-[calc(100vh-4rem)] flex flex-col items-center justify-center">
      <UCard class="w-full max-w-xl">
        <template #header>
          <h1 class="text-2xl font-semibold text-center">
            Modifier «&nbsp;{{ vehicle.brand }} {{ vehicle.model }}&nbsp;»
          </h1>
        </template>

        <UForm
          :schema="schema"
          :state="state"
          @submit="handleSubmit"
          class="space-y-4"
        >
          <UFormGroup
            label="Marque"
            name="brand"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.brand"
                :placeholder="vehicle.brand"
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
                  @click="state.brand = undefined"
                  :disabled="!state.brand"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>
          <UFormGroup
            label="Modèle"
            name="model"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.model"
                :placeholder="vehicle.model"
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
                  @click="state.model = undefined"
                  :disabled="!state.model"
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
              <UInput
                v-model="state.description"
                :placeholder="vehicle.description || 'Ex. : Voiture de service, 4x4 rouge…'"
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
                  @click="state.description = undefined"
                  :disabled="!state.description"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>
          <UFormGroup
            label="Immatriculation"
            name="registration"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.registration"
                :placeholder="vehicle.registration"
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
                  @click="state.registration = undefined"
                  :disabled="!state.registration"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>
          <UFormGroup
            label="Numéro de série"
            name="serialNumber"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.serialNumber"
                :placeholder="vehicle.serialNumber || ''"
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
                  @click="state.serialNumber = undefined"
                  :disabled="!state.serialNumber"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>
          <UFormGroup
            label="Année de première immatriculation"
            name="registrationYear"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.registrationYear"
                class="flex-1"
                type="number"
                min="1900"
                :max="new Date().getFullYear()"
                :placeholder="String(vehicle.registrationYear)"
              />
              <UTooltip
                :popper="{ arrow: true }"
                text="Réinitialiser"
              >
                <!--suppress PointlessBooleanExpressionJS -->
                <UButton
                  color="white"
                  class="size-8"
                  @click="state.registrationYear = 0"
                  :disabled="!state.registrationYear"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>
          <UFormGroup
            label="Compteur kilométrique"
            name="odometer"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.odometer"
                class="flex-1"
                type="number"
                min="1"
                :placeholder="String(vehicle.odometer)"
              />
              <UTooltip
                :popper="{ arrow: true }"
                text="Réinitialiser"
              >
                <!--suppress PointlessBooleanExpressionJS -->
                <UButton
                  color="white"
                  class="size-8"
                  @click="state.odometer = 0"
                  :disabled="!state.odometer"
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
              text="Supprimer le profil"
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
              Êtes-vous sûr de vouloir supprimer «&nbsp;{{ vehicle.brand }}
              {{ vehicle.model }}&nbsp;» ?
            </h3>

            <p class="text-neutral-400 text-sm text-pretty">
              Toutes les réparations associées au véhicule «&nbsp;{{ vehicle.brand }}
              {{ vehicle.model }}&nbsp;» immatriculé
              <span class="font-semibold">{{ vehicle.registration }}</span
              >, si elles n'ont pas été exportées, seront définitivement perdues.
            </p>

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
  </ProfileLayout>
</template>
