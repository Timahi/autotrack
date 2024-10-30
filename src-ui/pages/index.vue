<script lang="ts" setup>
const { $profileService, $importExportService } = useNuxtApp()

const toast = useToast()

const { data, refresh } = useAsyncData('profiles', () => $profileService.getAll())

const profiles = computed(() => data.value?.sort((a, b) => a.name.localeCompare(b.name)))

const importConfirmationModalOpen = ref(false)

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
  <div class="min-h-screen grid grid-rows-3 place-items-center mx-4">
    <h1 class="text-4xl font-semibold text-neutral-300">
      Auto<span class="font-bold text-primary">Track</span>
    </h1>

    <div
      id="profile-list"
      class="relative w-full max-w-screen-sm md:max-w-screen-md lg:max-w-screen-lg xl:max-w-screen-xl"
    >
      <div
        class="flex items-center gap-6 overflow-x-scroll whitespace-nowrap p-6"
        :class="{ 'justify-center': !profiles?.length }"
      >
        <!--suppress HtmlUnknownTarget -->
        <NuxtLink
          class="hover:scale-105 duration-200"
          v-for="profile of profiles"
          :key="profile.id"
          :to="`/profiles/${profile.id}`"
        >
          <UCard class="border border-primary size-52 flex items-center justify-center">
            <div class="grid grid-rows-1 place-items-center gap-y-3">
              <p class="text-xl">{{ profile.name }}</p>
            </div>
          </UCard>
        </NuxtLink>

        <UDivider
          orientation="vertical"
          size="sm"
          class="h-52"
          v-if="profiles?.length"
        />

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
        <button
          class="hover:scale-105 duration-200"
          @click="importConfirmationModalOpen = true"
        >
          <UCard
            class="border border-dashed border-neutral-500 size-52 flex items-center justify-center"
          >
            <div class="grid grid-rows-2 place-items-center gap-y-3">
              <p class="text-xl text-wrap">Importer une sauvegarde</p>
              <IDownload class="size-10" />
            </div>
          </UCard>
        </button>
        <button
          class="hover:scale-105 duration-200"
          @click="handleExport"
          v-if="profiles?.length"
        >
          <UCard
            class="border border-dashed border-neutral-500 size-52 flex items-center justify-center"
          >
            <div class="grid grid-rows-2 place-items-center gap-y-3">
              <p class="text-xl text-wrap">Exporter les données</p>
              <IUpload class="size-10" />
            </div>
          </UCard>
        </button>
      </div>
    </div>

    <UModal v-model="importConfirmationModalOpen">
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
              @click="importConfirmationModalOpen = false"
              block
            >
              Annuler
            </UButton>
            <UButton
              @click="
                () => {
                  importConfirmationModalOpen = false
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

<style scoped>
#profile-list::before {
  content: '';
  background: linear-gradient(90deg, rgba(30, 30, 30, 1) 0%, rgba(255, 255, 255, 0) 100%);
  height: 260px;
  width: 30px;
  position: absolute;
  z-index: 20;
  top: 0;
  left: 0;
}

#profile-list::after {
  content: '';
  background: linear-gradient(90deg, rgba(255, 255, 255, 0) 0%, rgba(30, 30, 30, 1) 100%);
  height: 260px;
  width: 30px;
  position: absolute;
  z-index: 20;
  right: 0;
  top: 0;
}
</style>
