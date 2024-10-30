<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

definePageMeta({
  middleware: 'profile-middleware',
  layout: 'profile-layout',
})

const { profile } = storeToRefs(selectedProfile())
const { $vehicleService } = useNuxtApp()

const schema = $vehicleService.schemas.createAndUpdate

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  brand: '',
  model: '',
  registration: '',
  registrationYear: 2000,
  odometer: 1,
  serialNumber: undefined,
  description: undefined,
})

const toast = useToast()
const loading = ref(false)
async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $vehicleService.create(profile.value.id, event.data)
    await navigateTo(`/profiles/${profile.value.id}/vehicles`)
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'create_vehicle_error', title: error })
    } else {
      toast.add({ id: 'create_vehicle_error', title: 'Une erreur est survenue' })
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center">
    <UCard class="w-full max-w-xl">
      <template #header>
        <h1 class="text-2xl font-semibold text-center">Créer un véhicule</h1>
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
          required
        >
          <UInput v-model="state.brand" />
        </UFormGroup>
        <UFormGroup
          label="Modèle"
          name="model"
          required
        >
          <UInput v-model="state.model" />
        </UFormGroup>
        <UFormGroup
          label="Description"
          name="description"
        >
          <UInput
            v-model="state.description"
            placeholder="Ex. : Voiture de service, 4x4 rouge…"
          />
        </UFormGroup>
        <UFormGroup
          label="Immatriculation"
          name="registration"
          required
        >
          <UInput v-model="state.registration" />
        </UFormGroup>
        <UFormGroup
          label="Numéro de série"
          name="serialNumber"
        >
          <UInput v-model="state.serialNumber" />
        </UFormGroup>
        <UFormGroup
          label="Année de première immatriculation"
          name="registrationYear"
          required
        >
          <UInput
            v-model="state.registrationYear"
            type="number"
            min="1900"
            :max="new Date().getFullYear()"
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
</template>
