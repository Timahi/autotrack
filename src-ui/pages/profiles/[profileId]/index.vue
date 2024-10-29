<script lang="ts" setup>
definePageMeta({
  middleware: ['profile-middleware'],
  layout: 'profile-layout',
})

const { profile } = storeToRefs(selectedProfile())
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
    <h1 class="text-4xl font-semibold">{{ profile.name }}</h1>
    <p>{{ profile.createdAt }}</p>
  </div>
</template>
