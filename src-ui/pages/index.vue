<script lang="ts" setup>
const { $profileService } = useNuxtApp()

const { data } = useAsyncData('profiles', () => $profileService.getAll())
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

    <div class="flex items-center gap-6">
      <UButton
        size="lg"
        variant="outline"
      >
        <template #leading>
          <IDownload class="size-5" />
        </template>
        Importer une sauvegarde
      </UButton>

      <UButton
        size="lg"
        variant="outline"
      >
        <template #leading>
          <IUpload class="size-5" />
        </template>
        Exporter la sauvegarde
      </UButton>
    </div>
  </div>
</template>
