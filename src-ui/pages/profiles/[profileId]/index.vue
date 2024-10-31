<script lang="ts" setup>
const profile = await useProfile()
</script>

<template>
  <ProfileLayout>
    <div class="min-h-screen flex flex-col items-center justify-center">
      <pre>{{ profile }}</pre>
    </div>
  </ProfileLayout>
</template>
