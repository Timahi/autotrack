<script lang="ts" setup>
import type { DropdownItem } from '#ui/types'

const { $profileService } = useNuxtApp()

const { profile } = storeToRefs(selectedProfile())
const { data: profiles } = useAsyncData('profiles', () => $profileService.getAll())

const items = computed<DropdownItem[][]>(() => [
  (profiles.value || []).map((p) => ({
    label: p.name,
    disabled: p.id === profile.value!.id,
    href: `/profiles/${p.id}`,
  })),
])
</script>

<template>
  <div class="min-h-screen">
    <header class="h-16 p-2 border-b border-neutral-600">
      <UDropdown
        mode="click"
        :items="items"
        :popper="{ arrow: true, placement: 'bottom-start' }"
        :ui="{ width: 'w-56' }"
      >
        <UButton
          color="white"
          size="lg"
          class="w-56"
        >
          <div class="flex-1 text-base truncate">{{ profile!.name }}</div>
          <template #trailing>
            <IChevronDown class="size-6 text-end" />
          </template>
        </UButton>
      </UDropdown>
    </header>
    <slot></slot>
  </div>
</template>
