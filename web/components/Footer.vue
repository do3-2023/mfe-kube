<template>
  <footer class="flex gap-10 mx-auto text-sm font-base">
    <p v-if="data?.hostname">
      Instance:
      <span class="ml-1 badge badge-sm">{{ data.hostname }}</span>
    </p>

    <a class="link link-hover" href="https://github.com/do3-2023/mfe-kube" target="_blank">
      Github
      <Icon name="fe:link-external" size="15px" />
    </a>
  </footer>
</template>

<script setup lang="ts">
const { data } = await useFetch('/api/hostname');
</script>
