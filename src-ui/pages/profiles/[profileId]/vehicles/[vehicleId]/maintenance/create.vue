<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

const profile = await useProfile()
const vehicle = await useVehicle()
const { $maintenanceService } = useNuxtApp()

const schema = $maintenanceService.schemas.create

type Values = z.infer<typeof schema>

const state = computed(() =>
  reactive<Values>({
    type: '',
    description: undefined,
    odometer: vehicle.value.odometer,
    performedAt: new Date(),
  })
)

const toast = useToast()
const loading = ref(false)
async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $maintenanceService.create(vehicle.value.id, event.data)
    await navigateTo(`/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/maintenance`)
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'create-maintenance-error', title: error })
    } else {
      toast.add({ id: 'create-maintenance-error', title: 'Une erreur est survenue' })
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <ProfileLayout>
    <VehicleLayout>
      <div class="h-[calc(100vh-9rem-48px)] flex flex-col items-center justify-center">
        <UCard class="w-full max-w-xl">
          <template #header>
            <h1 class="text-2xl font-semibold text-center">Ajouter un entretien</h1>
          </template>

          <UForm
            :schema="schema"
            :state="state"
            @submit="handleSubmit"
            class="space-y-4"
          >
            <UFormGroup
              label="Nature du travail"
              name="type"
              required
            >
              <UInput
                v-model="state.type"
                placeholder="Ex. : Révision, Remplacement de pièce, Vidange…"
              />
            </UFormGroup>
            <UFormGroup
              label="Description"
              name="description"
            >
              <UTextarea
                v-model="state.description"
                placeholder="Indiquez le détail de votre travail…"
              />
            </UFormGroup>
            <UFormGroup
              label="Compteur kilométrique"
              name="odometer"
              required
            >
              <UInput
                v-model="state.odometer"
                type="number"
                min="1"
              />
            </UFormGroup>

            <UFormGroup
              label="Réalisé le"
              name="performedAt"
              required
            >
              <UPopover :popper="{ placement: 'top' }">
                <UButton
                  icon="i-lucide-calendar-days"
                  :label="formatFullDate(state.performedAt)"
                  color="white"
                  block
                />

                <template #panel="{ close }">
                  <DatePicker
                    v-model="state.performedAt"
                    is-required
                    @close="close"
                  />
                </template>
              </UPopover>
            </UFormGroup>

            <UButton
              type="submit"
              :loading="loading"
              block
            >
              Sauvegarder
            </UButton>
          </UForm>
        </UCard>
      </div>
    </VehicleLayout>
  </ProfileLayout>
</template>
