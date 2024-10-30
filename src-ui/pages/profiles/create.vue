<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

const { $profileService } = useNuxtApp()

const schema = $profileService.schemas.create

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  name: '',
})

const toast = useToast()
const loading = ref(false)
async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  loading.value = true
  try {
    await $profileService.create(event.data)
    await navigateTo('/')
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'create_post_error', title: error })
    } else {
      toast.add({ id: 'create_post_error', title: 'Une erreur est survenue' })
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center relative">
    <div class="absolute top-0 left-0 m-5">
      <UButton
        variant="soft"
        to="/"
      >
        <template #leading>
          <IChevronsLeft class="size-4" />
        </template>
        Retour à la liste
      </UButton>
    </div>

    <UCard class="w-full max-w-xl">
      <template #header>
        <h1 class="text-2xl font-semibold text-center">Créer un profil</h1>
      </template>

      <UForm
        :schema="schema"
        :state="state"
        @submit="handleSubmit"
        class="space-y-4"
      >
        <UFormGroup
          label="Nom du profil"
          name="name"
        >
          <UInput
            v-model="state.name"
            placeholder="Ex. : Mes voitures perso, Garage n°2…"
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
