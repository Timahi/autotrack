<script lang="ts" setup>
const { $profileService, $importExportService } = useNuxtApp()

const { data, refresh } = useAsyncData('profiles', () => $profileService.getAll())

const toast = useToast()

const isImportConfirmationModalOpen = ref(false)

async function handleImport() {
  try {
    await $importExportService.import()
    toast.add({ id: 'import_success', title: 'Données importées avec succès !', color: 'green' })
    await refresh()
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'import_fail', title: error })
    } else {
      toast.add({ id: 'import_fail', title: 'Une erreur est survenue' })
    }
  }
}

async function handleExport() {
  try {
    await $importExportService.export()
    toast.add({ id: 'export_success', title: 'Données exportées avec succès !', color: 'green' })
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'export_fail', title: error })
    } else {
      toast.add({ id: 'export_fail', title: 'Une erreur est survenue' })
    }
  }
}
</script>

<template>
  <div class="min-h-screen grid grid-rows-3 place-items-center text-neutral-300 mx-4">
    <h1 class="text-4xl font-semibold">Sélectionner un profil</h1>
    <div class="flex flex-wrap items-center gap-6">
      <button
        class="hover:scale-105 duration-200"
        v-for="profile of data"
        :key="profile.id"
      >
        <UCard class="border border-neutral-500 size-52 flex items-center justify-center">
          <div class="grid grid-rows-2 place-items-center gap-y-3">
            <p class="text-xl">{{ profile.name }}</p>
          </div>
        </UCard>
      </button>

      <NuxtLink
        class="hover:scale-105 duration-200"
        to="/profiles/create"
      >
        <UCard
          class="border border-dashed border-neutral-500 size-52 flex items-center justify-center"
        >
          <div class="grid grid-rows-2 place-items-center gap-y-3">
            <p class="text-xl">Nouveau profil</p>
            <ISquarePlus class="size-10" />
          </div>
        </UCard>
      </NuxtLink>
    </div>

    <div class="grid grid-cols-2 gap-6">
      <UButton
        size="lg"
        variant="outline"
        @click="isImportConfirmationModalOpen = true"
      >
        <template #leading>
          <IDownload class="size-5" />
        </template>
        Importer une sauvegarde
      </UButton>

      <UButton
        size="lg"
        variant="outline"
        @click="handleExport"
      >
        <template #leading>
          <IUpload class="size-5" />
        </template>
        Exporter la sauvegarde
      </UButton>
    </div>

    <UModal v-model="isImportConfirmationModalOpen">
      <UCard>
        <div class="space-y-4">
          <h3 class="font-semibold text-xl text-center text-balance">
            Vérifiez que vous avez bien exporté vos données actuelles !
          </h3>

          <p class="text-neutral-400 text-sm font-thin flex items-center">
            <IAlertTriangle class="size-4 mr-2" />
            Les données non exportées seront écrasées
          </p>

          <div class="grid grid-cols-2 gap-4">
            <UButton
              variant="soft"
              @click="isImportConfirmationModalOpen = false"
              block
            >
              Annuler
            </UButton>
            <UButton
              @click="
                () => {
                  isImportConfirmationModalOpen = false
                  handleImport()
                }
              "
              block
            >
              Poursuivre
            </UButton>
          </div>
        </div>
      </UCard>
    </UModal>
  </div>
</template>
