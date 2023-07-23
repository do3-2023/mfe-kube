import { useLogger } from '@nuxt/kit';

const config = useRuntimeConfig();
const logger = useLogger();

export default defineEventHandler(async () => {
  const data = await $fetch(`${config.public.apiBase}/messages`, { method: 'GET' })
    .then((res) => {
      logger.info(
        `${new Date().toISOString()} | INFO method=GET uri=${config.public.apiBase}/messages response="${
          (res as Array<never>).length
        } messages retrieved"`
      );

      return res;
    })
    .catch((e) => {
      logger.error(`${new Date().toISOString()} | ERROR method=GET uri=${config.public.apiBase}/messages`, e);

      return [];
    });

  return data;
});
