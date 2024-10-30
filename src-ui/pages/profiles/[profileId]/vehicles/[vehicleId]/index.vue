<script lang="ts" setup>
const vehicle = await useVehicle()
</script>

<template>
  <ProfileLayout>
    <div class="min-h-screen flex flex-col items-center justify-center">
      <pre>{{ vehicle }}</pre>
      <p>{{ vehicle?.odometerUpdatedAt }}</p>
    </div>
  </ProfileLayout>
</template>
