<template>
  <div class="flex-1 flex flex-col-reverse gap-1 overflow-y-scroll mb-4">
    <MessageItem v-for="message in data" :key="message.id" :message="message" />
  </div>
</template>

<script setup lang="ts">
const { data, refresh } = await useFetch<
  {
    id: string;
    content: string;
    createdAt: string;
  }[]
>('/api/messages', { method: 'GET' });

setInterval(() => refresh(), 500);
</script>
