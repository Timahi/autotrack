<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

definePageMeta({
  middleware: ['profile-middleware'],
  layout: 'profile-layout',
})

const { profile } = storeToRefs(selectedProfile())
const { setProfile } = selectedProfile()

const dateFormater = Intl.DateTimeFormat('fr-FR', {
  day: '2-digit',
  month: '2-digit',
  year: 'numeric',
  timeZone: 'Europe/Paris',
})
const timeFormater = Intl.DateTimeFormat('fr-FR', {
  hour: '2-digit',
  minute: '2-digit',
  timeZone: 'Europe/Paris',
})

const updatedAtDate = computed(() => dateFormater.format(profile.value.updatedAt))
const updatedAtTime = computed(() => timeFormater.format(profile.value.updatedAt))

const { $profileService } = useNuxtApp()

const schema = $profileService.schemas.update

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  name: '',
})

const toast = useToast()

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()

  try {
    const result = await $profileService.update(profile.value.id, event.data)
    setProfile(result)
    toast.add({ id: 'profile-update-success', title: 'Le profil a été mis à jour', color: 'green' })
    await refreshNuxtData(['profiles'])
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'profile-update-error', title: error })
    } else {
      toast.add({ id: 'profile-update-error', title: 'Une erreur est survenue' })
    }
  }
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center text-neutral-300">
    <UCard class="w-full max-w-xl">
      <template #header>
        <h1 class="text-2xl font-semibold text-center truncate">
          Modifier «&nbsp;{{ profile.name }}&nbsp;»
        </h1>
      </template>

      <UForm
        :schema="schema"
        :state="state"
        @submit="handleSubmit"
        class="space-y-4"
      >
        <UFormGroup
          label="Nouveau nom du profil"
          name="name"
        >
          <UInput
            v-model="state.name"
            :placeholder="profile.name"
          />
        </UFormGroup>

        <UButton
          type="submit"
          :disabled="state.name === profile.name"
          block
          >Mettre à jour le profil</UButton
        >
      </UForm>

      <template #footer>
        <p class="text-sm text-neutral-500">
          Dernière modification le {{ updatedAtDate }} à {{ updatedAtTime }}
        </p>
      </template>
    </UCard>
  </div>
</template>
