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

const deleteModalOpen = ref(false)

async function handleDelete() {
  try {
    await $profileService.delete(profile.value.id)
    toast.add({ id: 'profile-delete-success', title: 'Le profil a été supprimé', color: 'green' })
    await refreshNuxtData(['profiles'])
    await navigateTo('/')
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'profile-delete-error', title: error })
    } else {
      toast.add({ id: 'profile-delete-error', title: 'Une erreur est survenue' })
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
          >Mettre à jour le profil
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
            Êtes-vous sûr de vouloir supprimer «&nbsp;{{ profile.name }}&nbsp;» ?
          </h3>

          <p class="text-neutral-400 text-sm flex items-center text-pretty">
            <!--            <IAlertTriangle class="size-4 mr-2" />-->
            Tous les véhicules présents dans «&nbsp;{{ profile.name }}&nbsp;» seront supprimés s'ils
            n'ont pas été exportés.
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
</template>
