<script lang="ts" setup>
const route = useRoute()
const { $maintenanceService } = useNuxtApp()
const { data } = useAsyncData(`maintenance-${route.params.maintenanceId}`, () =>
  $maintenanceService.getById(+route.params.maintenanceId)
)
</script>

<template>
  <ProfileLayout>
    <VehicleLayout>
      <pre>{{ data }}</pre>
    </VehicleLayout>
  </ProfileLayout>
</template>
