<template>
  <div class="flex flex-col items-center justify-center gap-2 m-4 min-h-full">
    <span v-if="error" class="text-red-500">
      {{ error.message }}
    </span>
    <!---->
    <div class="flex flex-col gap-4">
      <div
        class="flex flex-row gap-4"
        v-for="calendarDate in calendarItemsByDate.data || []"
      >
        <DialogRoot
          v-for="calendar in calendarDate"
          :key="calendar.id"
          :to="'/calendar/' + calendar.id"
          :title="`${calendar.day}.${calendar.month}.${calendar.year}`"
        >
          <DialogTrigger class="flex flex-col gap-4 items-center">
            <img
              :src="'/assets/' + calendar.thumbnail"
              v-if="calendar.thumbnail"
              alt="Tag Icon"
              class="max-h-32 max-w-32 rounded"
            />
            <h3>{{ calendar.day }}.{{ calendar.month }}.{{ calendar.year }} ({{  calendar.animationType }})</h3>
          </DialogTrigger>
          <DialogOverlay
            class="bg-black/30 data-[state=open]:animate-overlayShow fixed inset-0 z-30"
          />
          <DialogContent
            class="data-[state=open]:animate-contentShow fixed top-[50%] left-[50%] max-h-[85vh] w-[90vw] max-w-[450px] translate-x-[-50%] translate-y-[-50%] rounded-[6px] bg-white p-[25px] shadow-[hsl(206_22%_7%_/_35%)_0px_10px_38px_-10px,_hsl(206_22%_7%_/_20%)_0px_10px_20px_-15px] focus:outline-none z-[100]"
          >
            <video controls class="object-cover rounded-lg shadow-lg">
              <source
                :src="'/assets/' + calendar?.animation"
                type="video/mp4"
              />
              Your browser does not support the video tag.
            </video>
          </DialogContent>
        </DialogRoot>
      </div>
    </div>
    <div class="mt-auto">
      <button
        @click="changePage(-1)"
        class="p-1 text-xl mr-2"
        :class="{
          invisible: page == 0,
        }"
      >
        <
      </button>
      <span
        >Page {{ page + 1 }} of {{ Math.ceil(calendarItemsByDate.totalLen / 5) }} ({{
          calendarItemsByDate.totalLen
        }}
        items)</span
      >
      <button
        v-if="page + 1 < Math.ceil(calendarItemsByDate.totalLen / 5)"
        @click="changePage(1)"
        class="p-1 text-xl ml-2"
      >
        >
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { graphql } from "~/utils/gql/gql";
import { AnimationTypeEnum } from "~/utils/gql/graphql";
import {
  DialogContent,
  DialogOverlay,
  DialogRoot,
  DialogTrigger,
} from "reka-ui";

const route = useRoute();
const page = computed(() => parseInt(route.query.page?.toString() || "0"));

const dataQuery = graphql(`
  query getCalendarData {
    calendar(
      pagination: { page: { limit: 1000, page: 0 } }
      orderBy: { year: ASC, month: ASC, day: ASC }
    ) {
      nodes {
        id
        day
        month
        year
        animation
        thumbnail
        animationType
      }
    }
  }
`);

const { data, error, refresh } = await useAsyncQuery(dataQuery);

const calendarItemsByDate = computed(() => {
  const items: Record<
    string,
    {
      id: string;
      day: number;
      month: number;
      year: number;
      animation?: string | null;
      thumbnail?: string | null;
      animationType?: AnimationTypeEnum| null;
    }[]
  > = {};
  data.value?.calendar?.nodes.forEach((item) => {
    const dateKey = `${item.year}-${item.month}-${item.day}-${item.animationType}`;
    if (!items[dateKey]) {
      items[dateKey] = [];
    }
    items[dateKey].push(item);
  });
  return {
    data: Object.fromEntries(
      Object.entries(items).slice(5 * page.value, 5 * (page.value + 1))
    ),
    totalLen: Object.keys(items).length,
  };
});

async function changePage(increment: number) {
  await navigateTo("/calendars?page=" + (page.value + increment));
  await refresh();
}

definePageMeta({
  layout: "wrapper",
});
</script>
