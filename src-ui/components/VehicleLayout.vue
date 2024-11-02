<script lang="ts" setup>
import type { HorizontalNavigationLink } from '#ui/types'

const profile = await useProfile()
const vehicle = await useVehicle()

const route = useRoute()

const links: HorizontalNavigationLink[][] = [
  [
    {
      label: 'Entretien',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-maintenance',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/maintenance`
        )
      ).value,
      icon: 'i-lucide-wrench',
    },
    {
      label: 'Rappels',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-reminders',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/reminders`
        )
      ).value,
      icon: 'i-lucide-bell',
    },
    {
      label: 'Contrôle Technique',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-inspections',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      active: computed(() =>
        route.fullPath.startsWith(
          `/profiles/${profile.value.id}/vehicles/${vehicle.value.id}/inspections`
        )
      ).value,
      icon: 'i-lucide-clipboard-list',
    },
  ],

  [
    {
      label: 'Modifier',
      to: {
        name: 'profiles-profileId-vehicles-vehicleId-edit',
        params: { profileId: profile.value.id, vehicleId: vehicle.value.id },
      },
      icon: 'i-lucide-file-cog',
    },
  ],
]
</script>

<template>
  <div>
    <header class="px-10">
      <UHorizontalNavigation :links="links" />
    </header>

    <slot></slot>
  </div>
</template>
