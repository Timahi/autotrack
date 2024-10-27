<template>
  <div class="min-h-screen grid grid-rows-3 place-items-center text-neutral-300">
    <h1 class="text-4xl font-semibold">Sélectionner un profil</h1>
    <div class="flex items-center gap-6">
      <NuxtLink
        to="/profiles/create"
        class="hover:scale-105 duration-200"
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
      <button class="hover:scale-105 duration-200">
        <UCard
          class="border border-dashed border-neutral-500 size-52 flex items-center justify-center"
        >
          <div class="grid grid-rows-2 place-items-center gap-y-3">
            <p class="text-xl">Importer une sauvegarde</p>
            <IDownload class="size-10" />
          </div>
        </UCard>
      </button>
    </div>
  </div>
</template>
