<template>
  <div class="flex flex-col items-center justify-center gap-2 m-4 min-h-full">
    <span v-if="error" class="text-red-500">
      {{ error.message }}
    </span>
    <div class="flex flex-col gap-4">
      <div v-for="event in data?.eventSchedule?.nodes || []" :key="event.id" class="flex gap-4 items-center">
        <img
          :src="'/assets/' + event.icon"
          class="h-12 w-12 rounded"
        />
        <div>
          <h3 class="text-2xl">{{ event.name }}</h3>
          <p>{{ event.memo }}</p>
          <p>{{ event.categoryName }}</p>
          <!--<p>{{ event.largeCategory }}</p>-->
          <span>{{ new Date(event.startTime).toLocaleString() }}</span>
          <span v-if="new Date(event.endTime).getFullYear() != 9999">
            - {{ new Date(event.endTime).toLocaleString() }}</span
          >
          <p class="text-sm text-gray-500">
            Imported at: {{ new Date(event.importedAt).toLocaleString() }}
          </p>
        </div>
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
        >Page {{ page + 1 }} of
        {{ data?.eventSchedule?.paginationInfo?.pages }} ({{
          data?.eventSchedule?.paginationInfo?.total
        }}
        items)</span
      >
      <button
        v-if="page + 1 < (data?.eventSchedule?.paginationInfo?.pages || 0)"
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

const route = useRoute();
const page = computed(() => parseInt(route.query.page?.toString() || "0"));

const dataQuery = graphql(`
  query getEventData($page: Int!) {
    eventSchedule(
      pagination: { page: { limit: 7, page: $page } }
      orderBy: { startTime: ASC }
    ) {
      nodes {
        id
        name
        memo
        categoryName
        largeCategory
        startTime
        endTime
        icon
        importedAt
      }
      paginationInfo {
        pages
        total
      }
    }
  }
`);

const { data, error, refresh } = await useAsyncQuery(dataQuery, {
  page: page,
});

async function changePage(increment: number) {
  await navigateTo("/events?page=" + (page.value + increment));
  await refresh();
}

definePageMeta({
  layout: "wrapper",
});
</script>
