<script lang="ts" setup>
const profile = await useProfile()
const vehicle = await useVehicle()
const maintenance = await useMaintenance()
</script>

<template>
  <ProfileLayout>
    <VehicleLayout>
      <div class="h-[calc(100vh-9rem-48px)] flex flex-col items-center justify-center">
        <UCard class="w-full max-w-xl">
          <template #header>
            <h1 class="text-2xl font-semibold text-center">{{ maintenance!.type }}</h1>
          </template>

          <p class="whitespace-pre">{{ maintenance!.description }}</p>

          <template #footer>
            <div class="flex items-center justify-between">
              <div class="flex flex-col">
                <p class="text-sm text-neutral-400">
                  Réalisé le :
                  <span class="text-white">{{ formatDate(maintenance!.performedAt) }}</span>
                </p>
                <p class="text-sm text-neutral-400">
                  Relevé kilométrique :
                  <span class="text-white">{{ formatNumber(maintenance!.odometer) }}</span>
                </p>
              </div>

              <UTooltip
                text="Modifier l'entretien"
                :popper="{ arrow: true }"
              >
                <UButton
                  color="gray"
                  size="sm"
                  :to="`/profiles/${profile.id}/vehicles/${vehicle.id}/maintenance/${maintenance.id}/edit`"
                  square
                >
                  <IPencil class="size-5" />
                </UButton>
              </UTooltip>
            </div>
          </template>
        </UCard>
      </div>
    </VehicleLayout>
  </ProfileLayout>
</template>
