<template>
  <div class="chat chat-start">
    <div class="chat-header ml-1">
      <time class="text-xs opacity-50">{{ moment(props.message.createdAt).fromNow() }}</time>
    </div>
    <div class="chat-bubble">{{ props.message.content }}</div>
  </div>
</template>

<script setup lang="ts">
import moment from 'moment';

const props = defineProps<{
  message: {
    id: string;
    content: string;
    createdAt: string;
  };
}>();
</script>

<style scoped>
.chat-bubble {
  background-color: var(--fg-color);
}
</style>
