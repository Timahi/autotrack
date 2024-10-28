<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'
import { invoke } from '@tauri-apps/api/core'

const schema = z.object({
  name: z
    .string()
    .min(1, 'Le nom du profil est obligatoire')
    .max(200, 'Le nom du profil ne peut excéder 200 caractères'),
})

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  name: '',
})

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()

  const response = await invoke('create_profile_handler', { name: event.data.name })
  console.log(response)
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
