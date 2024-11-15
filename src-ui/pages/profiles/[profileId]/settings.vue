<script lang="ts" setup>
import { z } from 'zod'
import type { FormSubmitEvent } from '#ui/types'

const profile = await useProfile()

const updatedAtDate = computed(() => formatDate(profile.value.updatedAt))
const updatedAtTime = computed(() => formatTime(profile.value.updatedAt))

const { $profileService } = useNuxtApp()

const schema = $profileService.schemas.update

type Values = z.infer<typeof schema>

const state = reactive<Values>({
  name: undefined,
})

const noChanges = computed(() => {
  for (const value of Object.values(state)) {
    if (value) {
      return false
    }
  }

  return true
})

const toast = useToast()
const submitLoading = ref(false)

async function handleSubmit(event: FormSubmitEvent<Values>) {
  event.preventDefault()
  submitLoading.value = true
  try {
    await $profileService.update(profile.value.id, event.data)
    toast.add({ id: 'profile-update-success', title: 'Le profil a été mis à jour', color: 'green' })
    await refreshNuxtData(['profiles', `profile-${profile.value.id}`])
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'profile-update-error', title: error })
    } else {
      toast.add({ id: 'profile-update-error', title: 'Une erreur est survenue' })
    }
  } finally {
    for (const key of Object.keys(state)) {
      state[key as keyof typeof state] = undefined
    }
    submitLoading.value = false
  }
}

const deleteModalOpen = ref(false)
const deleteLoading = ref(false)

async function handleDelete() {
  deleteLoading.value = true
  try {
    await $profileService.delete(profile.value.id)
    toast.add({ id: 'profile-delete-success', title: 'Le profil a été supprimé', color: 'green' })
    await refreshNuxtData(['profiles', `profile-${profile.value.id}`])
    await navigateTo('/')
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'profile-delete-error', title: error })
    } else {
      toast.add({ id: 'profile-delete-error', title: 'Une erreur est survenue' })
    }
  } finally {
    deleteLoading.value = false
  }
}
</script>

<template>
  <ProfileLayout>
    <div class="min-h-screen flex flex-col items-center justify-center">
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
            label="Nom du profil"
            name="name"
          >
            <div class="flex items-center gap-2">
              <UInput
                v-model="state.name"
                :placeholder="profile.name"
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
                  @click="state.name = undefined"
                  :disabled="!state.name"
                >
                  <IRotateCcw />
                </UButton>
              </UTooltip>
            </div>
          </UFormGroup>

          <UButton
            type="submit"
            :disabled="noChanges"
            :loading="submitLoading"
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
              Êtes-vous sûr de vouloir supprimer «&nbsp;{{ profile.name }}&nbsp;» ?
            </h3>

            <p class="text-neutral-400 text-sm text-pretty">
              Tous les véhicules présents dans «&nbsp;{{ profile.name }}&nbsp;» seront supprimés
              s'ils n'ont pas été exportés.
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
