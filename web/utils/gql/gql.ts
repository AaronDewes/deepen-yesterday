/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "\n  query getCalendarData {\n    calendar(\n      pagination: { page: { limit: 1000, page: 0 } }\n      orderBy: { year: ASC, month: ASC, day: ASC }\n    ) {\n      nodes {\n        id\n        day\n        month\n        year\n        animation\n        thumbnail\n        animationType\n      }\n    }\n  }\n": typeof types.GetCalendarDataDocument,
    "\n  query getContent($id: String!) {\n    contentItem(filters: { id: { eq: $id } }) {\n      nodes {\n        id\n        title\n        contentBodyUrl\n        note\n        contentMovie\n        contentImages\n      }\n    }\n  }\n": typeof types.GetContentDocument,
    "\n  query getAllContents($page: Int!) {\n    contentItem(pagination: { page: { limit: 8, page: $page } }, orderBy: { importedAt: ASC }) {\n      nodes {\n        id\n        thumbnail\n        title\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": typeof types.GetAllContentsDocument,
    "\n  query getEventData($page: Int!) {\n    eventSchedule(\n      pagination: { page: { limit: 7, page: $page } }\n      orderBy: { startTime: ASC }\n    ) {\n      nodes {\n        id\n        name\n        memo\n        categoryName\n        largeCategory\n        startTime\n        endTime\n        icon\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": typeof types.GetEventDataDocument,
    "\n  query getTagContentsData($page: Int!, $tag: String) {\n    tag(filters: { id: { eq: $tag } }) {\n      nodes {\n        id\n        name\n        icon\n        searchHeaderImage\n        contentItemTag(pagination: { page: { limit: 8, page: $page } }) {\n          nodes {\n            contentItem {\n              id\n              thumbnail\n              title\n            }\n          }\n          paginationInfo {\n            pages\n            total\n          }\n        }\n      }\n    }\n  }\n": typeof types.GetTagContentsDataDocument,
    "\n  query getTagData($page: Int!) {\n    tag(\n      pagination: { page: { limit: 8, page: $page } }\n      orderBy: { name: ASC }\n    ) {\n      nodes {\n        id\n        name\n        icon\n        platformText\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": typeof types.GetTagDataDocument,
};
const documents: Documents = {
    "\n  query getCalendarData {\n    calendar(\n      pagination: { page: { limit: 1000, page: 0 } }\n      orderBy: { year: ASC, month: ASC, day: ASC }\n    ) {\n      nodes {\n        id\n        day\n        month\n        year\n        animation\n        thumbnail\n        animationType\n      }\n    }\n  }\n": types.GetCalendarDataDocument,
    "\n  query getContent($id: String!) {\n    contentItem(filters: { id: { eq: $id } }) {\n      nodes {\n        id\n        title\n        contentBodyUrl\n        note\n        contentMovie\n        contentImages\n      }\n    }\n  }\n": types.GetContentDocument,
    "\n  query getAllContents($page: Int!) {\n    contentItem(pagination: { page: { limit: 8, page: $page } }, orderBy: { importedAt: ASC }) {\n      nodes {\n        id\n        thumbnail\n        title\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": types.GetAllContentsDocument,
    "\n  query getEventData($page: Int!) {\n    eventSchedule(\n      pagination: { page: { limit: 7, page: $page } }\n      orderBy: { startTime: ASC }\n    ) {\n      nodes {\n        id\n        name\n        memo\n        categoryName\n        largeCategory\n        startTime\n        endTime\n        icon\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": types.GetEventDataDocument,
    "\n  query getTagContentsData($page: Int!, $tag: String) {\n    tag(filters: { id: { eq: $tag } }) {\n      nodes {\n        id\n        name\n        icon\n        searchHeaderImage\n        contentItemTag(pagination: { page: { limit: 8, page: $page } }) {\n          nodes {\n            contentItem {\n              id\n              thumbnail\n              title\n            }\n          }\n          paginationInfo {\n            pages\n            total\n          }\n        }\n      }\n    }\n  }\n": types.GetTagContentsDataDocument,
    "\n  query getTagData($page: Int!) {\n    tag(\n      pagination: { page: { limit: 8, page: $page } }\n      orderBy: { name: ASC }\n    ) {\n      nodes {\n        id\n        name\n        icon\n        platformText\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n": types.GetTagDataDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getCalendarData {\n    calendar(\n      pagination: { page: { limit: 1000, page: 0 } }\n      orderBy: { year: ASC, month: ASC, day: ASC }\n    ) {\n      nodes {\n        id\n        day\n        month\n        year\n        animation\n        thumbnail\n        animationType\n      }\n    }\n  }\n"): (typeof documents)["\n  query getCalendarData {\n    calendar(\n      pagination: { page: { limit: 1000, page: 0 } }\n      orderBy: { year: ASC, month: ASC, day: ASC }\n    ) {\n      nodes {\n        id\n        day\n        month\n        year\n        animation\n        thumbnail\n        animationType\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getContent($id: String!) {\n    contentItem(filters: { id: { eq: $id } }) {\n      nodes {\n        id\n        title\n        contentBodyUrl\n        note\n        contentMovie\n        contentImages\n      }\n    }\n  }\n"): (typeof documents)["\n  query getContent($id: String!) {\n    contentItem(filters: { id: { eq: $id } }) {\n      nodes {\n        id\n        title\n        contentBodyUrl\n        note\n        contentMovie\n        contentImages\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getAllContents($page: Int!) {\n    contentItem(pagination: { page: { limit: 8, page: $page } }, orderBy: { importedAt: ASC }) {\n      nodes {\n        id\n        thumbnail\n        title\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"): (typeof documents)["\n  query getAllContents($page: Int!) {\n    contentItem(pagination: { page: { limit: 8, page: $page } }, orderBy: { importedAt: ASC }) {\n      nodes {\n        id\n        thumbnail\n        title\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getEventData($page: Int!) {\n    eventSchedule(\n      pagination: { page: { limit: 7, page: $page } }\n      orderBy: { startTime: ASC }\n    ) {\n      nodes {\n        id\n        name\n        memo\n        categoryName\n        largeCategory\n        startTime\n        endTime\n        icon\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"): (typeof documents)["\n  query getEventData($page: Int!) {\n    eventSchedule(\n      pagination: { page: { limit: 7, page: $page } }\n      orderBy: { startTime: ASC }\n    ) {\n      nodes {\n        id\n        name\n        memo\n        categoryName\n        largeCategory\n        startTime\n        endTime\n        icon\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getTagContentsData($page: Int!, $tag: String) {\n    tag(filters: { id: { eq: $tag } }) {\n      nodes {\n        id\n        name\n        icon\n        searchHeaderImage\n        contentItemTag(pagination: { page: { limit: 8, page: $page } }) {\n          nodes {\n            contentItem {\n              id\n              thumbnail\n              title\n            }\n          }\n          paginationInfo {\n            pages\n            total\n          }\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query getTagContentsData($page: Int!, $tag: String) {\n    tag(filters: { id: { eq: $tag } }) {\n      nodes {\n        id\n        name\n        icon\n        searchHeaderImage\n        contentItemTag(pagination: { page: { limit: 8, page: $page } }) {\n          nodes {\n            contentItem {\n              id\n              thumbnail\n              title\n            }\n          }\n          paginationInfo {\n            pages\n            total\n          }\n        }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query getTagData($page: Int!) {\n    tag(\n      pagination: { page: { limit: 8, page: $page } }\n      orderBy: { name: ASC }\n    ) {\n      nodes {\n        id\n        name\n        icon\n        platformText\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"): (typeof documents)["\n  query getTagData($page: Int!) {\n    tag(\n      pagination: { page: { limit: 8, page: $page } }\n      orderBy: { name: ASC }\n    ) {\n      nodes {\n        id\n        name\n        icon\n        platformText\n        importedAt\n      }\n      paginationInfo {\n        pages\n        total\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;