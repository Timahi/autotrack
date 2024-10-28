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

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()

  try {
    await $profileService.create(event.data)
    await navigateTo('/')
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'create_post_error', title: error })
    } else {
      toast.add({ id: 'create_post_error', title: 'Une erreur est survenue' })
    }
  }
}
</script>

<template>
  <div class="min-h-screen grid grid-rows-3 place-items-center text-neutral-300 relative">
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
    <h1 class="text-4xl font-semibold">Créer un profil</h1>

    <UForm
      :schema="schema"
      :state="state"
      @submit="handleSubmit"
      class="max-w-lg w-full space-y-4"
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
        block
        >Créer le profil</UButton
      >
    </UForm>
  </div>
</template>
