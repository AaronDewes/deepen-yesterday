<template>
  <div
    class="flex flex-col items-center justify-center gap-2 m-4 self-center justify-self-center"
  >
    <h1 class="text-3xl font-bold">{{ content.title }}</h1>
    <img
      v-for="img in content?.contentImages"
      :key="img"
      :src="'/assets/' + img"
      alt="content image"
      class="max-w-dvw max-h-dvh object-cover rounded-lg shadow-lg"
    />
    <video
      v-if="content?.contentMovie"
      controls
      class="max-w-dvw max-h-dvh object-cover rounded-lg shadow-lg"
    >
      <source :src="'/assets/' + content?.contentMovie" type="video/mp4" />
      Your browser does not support the video tag.
    </video>
    <iframe
      v-if="content?.contentBodyUrl"
      :src="content?.contentBodyUrl"
      class="max-w-dvw max-h-dvh object-cover rounded-lg shadow-lg min-h-[80dvh] min-w-[40dvw]"
      frameborder="0"
      allowfullscreen
      sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
    ></iframe>
    <span v-if="content?.note">{{ content?.note }}</span>
  </div>
</template>

<script setup lang="ts">
import { graphql } from "~/utils/gql";
const { params } = useRoute();

const contentQuery = graphql(`
  query getContent($id: String!) {
    contentItem(filters: { id: { eq: $id } }) {
      nodes {
        id
        title
        contentBodyUrl
        note
        contentMovie
        contentImages
      }
    }
  }
`);

const { data, error } = await useAsyncQuery(contentQuery, {
  id: params.content,
});

const content = computed(() => {
  return data.value?.contentItem.nodes[0];
});
definePageMeta({
  layout: "wrapper",
});
</script>
