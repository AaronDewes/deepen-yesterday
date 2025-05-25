<template>
  <div class="flex flex-col items-center justify-center gap-2 m-4">
    <span v-if="error" class="text-red-500">
      {{ error.message }}
    </span>
    <img
      v-if="data?.tag?.nodes[0].searchHeaderImage"
      :src="
        '/assets/' + data.tag.nodes[0].searchHeaderImage
      "
      alt="Tag Header Image"
      class="max-w-full h-48 object-cover mb-4"
    />
    <h1 class="font-bold text-2xl">{{  data?.tag.nodes[0].name }}</h1>
    <div class="flex flex-col gap-4">
      <NuxtLink
        v-for="item in data?.tag?.nodes[0].contentItemTag.nodes || []"
        :key="item.contentItem.id"
        class="flex gap-4 items-center"
        :to="'/content/' + item.contentItem.id"
      >
        <img
          :src="'/assets/' + item.contentItem.thumbnail"
          v-if="item.contentItem.thumbnail"
          alt="Tag Icon"
          class="max-h-32 max-w-32 rounded"
        />
        <div v-else>
          <img
            src="~/assets/img/question.webp"
            alt="Placeholder Icon"
            class="h-32 w-32 rounded"
          />
        </div>
        <div>
          <h3 class="text-2xl">{{ item.contentItem.title }}</h3>
        </div>
      </NuxtLink>
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
        {{ data?.tag?.nodes[0].contentItemTag.paginationInfo?.pages }} ({{
          data?.tag?.nodes[0].contentItemTag.paginationInfo?.total
        }}
        items)</span
      >
      <button
        v-if="
          page + 1 < data?.tag?.nodes[0].contentItemTag?.paginationInfo?.pages
        "
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
const { params } = useRoute();

const route = useRoute();
const page = computed(() => parseInt(route.query.page?.toString() || "0"));

const dataQuery = graphql(`
  query getTagContentsData($page: Int!, $tag: String) {
    tag(filters: { id: { eq: $tag } }) {
      nodes {
        id
        name
        icon
        searchHeaderImage
        contentItemTag(pagination: { page: { limit: 8, page: $page } }) {
          nodes {
            contentItem {
              id
              thumbnail
              title
            }
          }
          paginationInfo {
            pages
            total
          }
        }
      }
    }
  }
`);

const { data, error, refresh } = await useAsyncQuery(dataQuery, {
  page: page,
  tag: params.tag,
});

async function changePage(increment: number) {
  await navigateTo("/tags/" + params.tag + "?page=" + (page.value + increment));
  await refresh();
}

definePageMeta({
  layout: "wrapper",
});
</script>
