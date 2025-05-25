/* eslint-disable */
// This file is generated automatically. DO NOT EDIT IT MANUALLY.
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export enum AnimationTypeEnum {
  Birthday = 'birthday',
  Daily = 'daily',
  Event = 'event',
  Special = 'special'
}

export type AnimationTypeEnumFilterInput = {
  eq?: InputMaybe<AnimationTypeEnum>;
  gt?: InputMaybe<AnimationTypeEnum>;
  gte?: InputMaybe<AnimationTypeEnum>;
  is_in?: InputMaybe<Array<AnimationTypeEnum>>;
  is_not_in?: InputMaybe<Array<AnimationTypeEnum>>;
  is_not_null?: InputMaybe<AnimationTypeEnum>;
  is_null?: InputMaybe<AnimationTypeEnum>;
  lt?: InputMaybe<AnimationTypeEnum>;
  lte?: InputMaybe<AnimationTypeEnum>;
  ne?: InputMaybe<AnimationTypeEnum>;
};

export type BooleanFilterInput = {
  eq?: InputMaybe<Scalars['Boolean']['input']>;
  gt?: InputMaybe<Scalars['Boolean']['input']>;
  gte?: InputMaybe<Scalars['Boolean']['input']>;
  is_in?: InputMaybe<Array<Scalars['Boolean']['input']>>;
  is_not_in?: InputMaybe<Array<Scalars['Boolean']['input']>>;
  is_not_null?: InputMaybe<Scalars['Boolean']['input']>;
  is_null?: InputMaybe<Scalars['Boolean']['input']>;
  lt?: InputMaybe<Scalars['Boolean']['input']>;
  lte?: InputMaybe<Scalars['Boolean']['input']>;
  ne?: InputMaybe<Scalars['Boolean']['input']>;
};

export type Calendar = {
  __typename?: 'Calendar';
  animation?: Maybe<Scalars['String']['output']>;
  animationType?: Maybe<AnimationTypeEnum>;
  dateColor?: Maybe<Scalars['String']['output']>;
  day: Scalars['Int']['output'];
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  month: Scalars['Int']['output'];
  repeatType?: Maybe<RepeatTypeEnum>;
  skinIp?: Maybe<SkinIpEnum>;
  thumbnail?: Maybe<Scalars['String']['output']>;
  year: Scalars['Int']['output'];
};

export type CalendarBasic = {
  __typename?: 'CalendarBasic';
  animation?: Maybe<Scalars['String']['output']>;
  animationType?: Maybe<AnimationTypeEnum>;
  dateColor?: Maybe<Scalars['String']['output']>;
  day: Scalars['Int']['output'];
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  month: Scalars['Int']['output'];
  repeatType?: Maybe<RepeatTypeEnum>;
  skinIp?: Maybe<SkinIpEnum>;
  thumbnail?: Maybe<Scalars['String']['output']>;
  year: Scalars['Int']['output'];
};

export type CalendarConnection = {
  __typename?: 'CalendarConnection';
  edges: Array<CalendarEdge>;
  nodes: Array<Calendar>;
  pageInfo: PageInfo;
  paginationInfo?: Maybe<PaginationInfo>;
};

export type CalendarEdge = {
  __typename?: 'CalendarEdge';
  cursor: Scalars['String']['output'];
  node: Calendar;
};

export type CalendarFilterInput = {
  and?: InputMaybe<Array<CalendarFilterInput>>;
  animation?: InputMaybe<StringFilterInput>;
  animationType?: InputMaybe<AnimationTypeEnumFilterInput>;
  dateColor?: InputMaybe<StringFilterInput>;
  day?: InputMaybe<IntegerFilterInput>;
  id?: InputMaybe<StringFilterInput>;
  importedAt?: InputMaybe<TextFilterInput>;
  month?: InputMaybe<IntegerFilterInput>;
  or?: InputMaybe<Array<CalendarFilterInput>>;
  repeatType?: InputMaybe<RepeatTypeEnumFilterInput>;
  skinIp?: InputMaybe<SkinIpEnumFilterInput>;
  thumbnail?: InputMaybe<StringFilterInput>;
  year?: InputMaybe<IntegerFilterInput>;
};

export type CalendarInsertInput = {
  animation?: InputMaybe<Scalars['String']['input']>;
  animationType?: InputMaybe<AnimationTypeEnum>;
  dateColor?: InputMaybe<Scalars['String']['input']>;
  day: Scalars['Int']['input'];
  id: Scalars['String']['input'];
  importedAt: Scalars['String']['input'];
  month: Scalars['Int']['input'];
  repeatType?: InputMaybe<RepeatTypeEnum>;
  skinIp?: InputMaybe<SkinIpEnum>;
  thumbnail?: InputMaybe<Scalars['String']['input']>;
  year: Scalars['Int']['input'];
};

export type CalendarOrderInput = {
  animation?: InputMaybe<OrderByEnum>;
  animationType?: InputMaybe<OrderByEnum>;
  dateColor?: InputMaybe<OrderByEnum>;
  day?: InputMaybe<OrderByEnum>;
  id?: InputMaybe<OrderByEnum>;
  importedAt?: InputMaybe<OrderByEnum>;
  month?: InputMaybe<OrderByEnum>;
  rawData?: InputMaybe<OrderByEnum>;
  repeatType?: InputMaybe<OrderByEnum>;
  skinIp?: InputMaybe<OrderByEnum>;
  thumbnail?: InputMaybe<OrderByEnum>;
  year?: InputMaybe<OrderByEnum>;
};

export type CalendarUpdateInput = {
  animation?: InputMaybe<Scalars['String']['input']>;
  animationType?: InputMaybe<AnimationTypeEnum>;
  dateColor?: InputMaybe<Scalars['String']['input']>;
  day?: InputMaybe<Scalars['Int']['input']>;
  id?: InputMaybe<Scalars['String']['input']>;
  importedAt?: InputMaybe<Scalars['String']['input']>;
  month?: InputMaybe<Scalars['Int']['input']>;
  repeatType?: InputMaybe<RepeatTypeEnum>;
  skinIp?: InputMaybe<SkinIpEnum>;
  thumbnail?: InputMaybe<Scalars['String']['input']>;
  year?: InputMaybe<Scalars['Int']['input']>;
};

export type ContentItem = {
  __typename?: 'ContentItem';
  category: Scalars['String']['output'];
  categoryColor?: Maybe<Scalars['String']['output']>;
  contentBodyUrl?: Maybe<Scalars['String']['output']>;
  contentImages?: Maybe<Array<Scalars['String']['output']>>;
  contentItemTag: ContentItemTagConnection;
  contentMovie?: Maybe<Scalars['String']['output']>;
  contentUrl?: Maybe<Scalars['String']['output']>;
  eventSchedule?: Maybe<EventSchedule>;
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  isNews: Scalars['Boolean']['output'];
  isPremiere: Scalars['Boolean']['output'];
  linkedEventScheduleId?: Maybe<Scalars['String']['output']>;
  note?: Maybe<Scalars['String']['output']>;
  panel?: Maybe<Scalars['String']['output']>;
  panelHeight: Scalars['Int']['output'];
  panelWidth: Scalars['Int']['output'];
  thumbnail?: Maybe<Scalars['String']['output']>;
  title: Scalars['String']['output'];
  titleColor?: Maybe<TitleColorEnum>;
};


export type ContentItemContentItemTagArgs = {
  filters?: InputMaybe<ContentItemTagFilterInput>;
  orderBy?: InputMaybe<ContentItemTagOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};

export type ContentItemBasic = {
  __typename?: 'ContentItemBasic';
  category: Scalars['String']['output'];
  categoryColor?: Maybe<Scalars['String']['output']>;
  contentBodyUrl?: Maybe<Scalars['String']['output']>;
  contentImages?: Maybe<Array<Scalars['String']['output']>>;
  contentMovie?: Maybe<Scalars['String']['output']>;
  contentUrl?: Maybe<Scalars['String']['output']>;
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  isNews: Scalars['Boolean']['output'];
  isPremiere: Scalars['Boolean']['output'];
  linkedEventScheduleId?: Maybe<Scalars['String']['output']>;
  note?: Maybe<Scalars['String']['output']>;
  panel?: Maybe<Scalars['String']['output']>;
  panelHeight: Scalars['Int']['output'];
  panelWidth: Scalars['Int']['output'];
  thumbnail?: Maybe<Scalars['String']['output']>;
  title: Scalars['String']['output'];
  titleColor?: Maybe<TitleColorEnum>;
};

export type ContentItemConnection = {
  __typename?: 'ContentItemConnection';
  edges: Array<ContentItemEdge>;
  nodes: Array<ContentItem>;
  pageInfo: PageInfo;
  paginationInfo?: Maybe<PaginationInfo>;
};

export type ContentItemEdge = {
  __typename?: 'ContentItemEdge';
  cursor: Scalars['String']['output'];
  node: ContentItem;
};

export type ContentItemFilterInput = {
  and?: InputMaybe<Array<ContentItemFilterInput>>;
  category?: InputMaybe<StringFilterInput>;
  categoryColor?: InputMaybe<StringFilterInput>;
  contentBodyUrl?: InputMaybe<StringFilterInput>;
  contentMovie?: InputMaybe<StringFilterInput>;
  contentUrl?: InputMaybe<StringFilterInput>;
  icon?: InputMaybe<StringFilterInput>;
  id?: InputMaybe<TextFilterInput>;
  importedAt?: InputMaybe<TextFilterInput>;
  isNews?: InputMaybe<BooleanFilterInput>;
  isPremiere?: InputMaybe<BooleanFilterInput>;
  linkedEventScheduleId?: InputMaybe<TextFilterInput>;
  note?: InputMaybe<StringFilterInput>;
  or?: InputMaybe<Array<ContentItemFilterInput>>;
  panel?: InputMaybe<StringFilterInput>;
  panelHeight?: InputMaybe<IntegerFilterInput>;
  panelWidth?: InputMaybe<IntegerFilterInput>;
  thumbnail?: InputMaybe<StringFilterInput>;
  title?: InputMaybe<StringFilterInput>;
  titleColor?: InputMaybe<TitleColorEnumFilterInput>;
};

export type ContentItemInsertInput = {
  category: Scalars['String']['input'];
  categoryColor?: InputMaybe<Scalars['String']['input']>;
  contentBodyUrl?: InputMaybe<Scalars['String']['input']>;
  contentImages?: InputMaybe<Array<Scalars['String']['input']>>;
  contentMovie?: InputMaybe<Scalars['String']['input']>;
  contentUrl?: InputMaybe<Scalars['String']['input']>;
  icon?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  importedAt: Scalars['String']['input'];
  isNews: Scalars['Boolean']['input'];
  isPremiere: Scalars['Boolean']['input'];
  linkedEventScheduleId?: InputMaybe<Scalars['String']['input']>;
  note?: InputMaybe<Scalars['String']['input']>;
  panel?: InputMaybe<Scalars['String']['input']>;
  panelHeight: Scalars['Int']['input'];
  panelWidth: Scalars['Int']['input'];
  thumbnail?: InputMaybe<Scalars['String']['input']>;
  title: Scalars['String']['input'];
  titleColor?: InputMaybe<TitleColorEnum>;
};

export type ContentItemOrderInput = {
  category?: InputMaybe<OrderByEnum>;
  categoryColor?: InputMaybe<OrderByEnum>;
  contentBodyUrl?: InputMaybe<OrderByEnum>;
  contentImages?: InputMaybe<OrderByEnum>;
  contentMovie?: InputMaybe<OrderByEnum>;
  contentUrl?: InputMaybe<OrderByEnum>;
  icon?: InputMaybe<OrderByEnum>;
  id?: InputMaybe<OrderByEnum>;
  importedAt?: InputMaybe<OrderByEnum>;
  isNews?: InputMaybe<OrderByEnum>;
  isPremiere?: InputMaybe<OrderByEnum>;
  linkedEventScheduleId?: InputMaybe<OrderByEnum>;
  note?: InputMaybe<OrderByEnum>;
  panel?: InputMaybe<OrderByEnum>;
  panelHeight?: InputMaybe<OrderByEnum>;
  panelWidth?: InputMaybe<OrderByEnum>;
  rawData?: InputMaybe<OrderByEnum>;
  thumbnail?: InputMaybe<OrderByEnum>;
  title?: InputMaybe<OrderByEnum>;
  titleColor?: InputMaybe<OrderByEnum>;
};

export type ContentItemTag = {
  __typename?: 'ContentItemTag';
  contentItem?: Maybe<ContentItem>;
  contentItemId: Scalars['String']['output'];
  id: Scalars['String']['output'];
  tag?: Maybe<Tag>;
  tagId: Scalars['String']['output'];
};

export type ContentItemTagBasic = {
  __typename?: 'ContentItemTagBasic';
  contentItemId: Scalars['String']['output'];
  id: Scalars['String']['output'];
  tagId: Scalars['String']['output'];
};

export type ContentItemTagConnection = {
  __typename?: 'ContentItemTagConnection';
  edges: Array<ContentItemTagEdge>;
  nodes: Array<ContentItemTag>;
  pageInfo: PageInfo;
  paginationInfo?: Maybe<PaginationInfo>;
};

export type ContentItemTagEdge = {
  __typename?: 'ContentItemTagEdge';
  cursor: Scalars['String']['output'];
  node: ContentItemTag;
};

export type ContentItemTagFilterInput = {
  and?: InputMaybe<Array<ContentItemTagFilterInput>>;
  contentItemId?: InputMaybe<TextFilterInput>;
  id?: InputMaybe<TextFilterInput>;
  or?: InputMaybe<Array<ContentItemTagFilterInput>>;
  tagId?: InputMaybe<StringFilterInput>;
};

export type ContentItemTagInsertInput = {
  contentItemId: Scalars['String']['input'];
  id: Scalars['String']['input'];
  tagId: Scalars['String']['input'];
};

export type ContentItemTagOrderInput = {
  contentItemId?: InputMaybe<OrderByEnum>;
  id?: InputMaybe<OrderByEnum>;
  tagId?: InputMaybe<OrderByEnum>;
};

export type ContentItemTagUpdateInput = {
  contentItemId?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['String']['input']>;
  tagId?: InputMaybe<Scalars['String']['input']>;
};

export type ContentItemUpdateInput = {
  category?: InputMaybe<Scalars['String']['input']>;
  categoryColor?: InputMaybe<Scalars['String']['input']>;
  contentBodyUrl?: InputMaybe<Scalars['String']['input']>;
  contentImages?: InputMaybe<Array<Scalars['String']['input']>>;
  contentMovie?: InputMaybe<Scalars['String']['input']>;
  contentUrl?: InputMaybe<Scalars['String']['input']>;
  icon?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['String']['input']>;
  importedAt?: InputMaybe<Scalars['String']['input']>;
  isNews?: InputMaybe<Scalars['Boolean']['input']>;
  isPremiere?: InputMaybe<Scalars['Boolean']['input']>;
  linkedEventScheduleId?: InputMaybe<Scalars['String']['input']>;
  note?: InputMaybe<Scalars['String']['input']>;
  panel?: InputMaybe<Scalars['String']['input']>;
  panelHeight?: InputMaybe<Scalars['Int']['input']>;
  panelWidth?: InputMaybe<Scalars['Int']['input']>;
  thumbnail?: InputMaybe<Scalars['String']['input']>;
  title?: InputMaybe<Scalars['String']['input']>;
  titleColor?: InputMaybe<TitleColorEnum>;
};

export type CursorInput = {
  cursor?: InputMaybe<Scalars['String']['input']>;
  limit: Scalars['Int']['input'];
};

export type EventSchedule = {
  __typename?: 'EventSchedule';
  allDay: Scalars['Boolean']['output'];
  categoryName?: Maybe<Scalars['String']['output']>;
  contentItem: ContentItemConnection;
  endTime: Scalars['String']['output'];
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  isDisplayHomeView: Scalars['Boolean']['output'];
  isMiddleDay: Scalars['Boolean']['output'];
  isUndefinedEndedAt: Scalars['Boolean']['output'];
  largeCategory?: Maybe<Scalars['String']['output']>;
  memo?: Maybe<Scalars['String']['output']>;
  name: Scalars['String']['output'];
  startTime: Scalars['String']['output'];
};


export type EventScheduleContentItemArgs = {
  filters?: InputMaybe<ContentItemFilterInput>;
  orderBy?: InputMaybe<ContentItemOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};

export type EventScheduleBasic = {
  __typename?: 'EventScheduleBasic';
  allDay: Scalars['Boolean']['output'];
  categoryName?: Maybe<Scalars['String']['output']>;
  endTime: Scalars['String']['output'];
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  isDisplayHomeView: Scalars['Boolean']['output'];
  isMiddleDay: Scalars['Boolean']['output'];
  isUndefinedEndedAt: Scalars['Boolean']['output'];
  largeCategory?: Maybe<Scalars['String']['output']>;
  memo?: Maybe<Scalars['String']['output']>;
  name: Scalars['String']['output'];
  startTime: Scalars['String']['output'];
};

export type EventScheduleConnection = {
  __typename?: 'EventScheduleConnection';
  edges: Array<EventScheduleEdge>;
  nodes: Array<EventSchedule>;
  pageInfo: PageInfo;
  paginationInfo?: Maybe<PaginationInfo>;
};

export type EventScheduleEdge = {
  __typename?: 'EventScheduleEdge';
  cursor: Scalars['String']['output'];
  node: EventSchedule;
};

export type EventScheduleFilterInput = {
  allDay?: InputMaybe<BooleanFilterInput>;
  and?: InputMaybe<Array<EventScheduleFilterInput>>;
  categoryName?: InputMaybe<StringFilterInput>;
  endTime?: InputMaybe<TextFilterInput>;
  icon?: InputMaybe<StringFilterInput>;
  id?: InputMaybe<TextFilterInput>;
  importedAt?: InputMaybe<TextFilterInput>;
  isDisplayHomeView?: InputMaybe<BooleanFilterInput>;
  isMiddleDay?: InputMaybe<BooleanFilterInput>;
  isUndefinedEndedAt?: InputMaybe<BooleanFilterInput>;
  largeCategory?: InputMaybe<StringFilterInput>;
  memo?: InputMaybe<StringFilterInput>;
  name?: InputMaybe<StringFilterInput>;
  or?: InputMaybe<Array<EventScheduleFilterInput>>;
  startTime?: InputMaybe<TextFilterInput>;
};

export type EventScheduleInsertInput = {
  allDay: Scalars['Boolean']['input'];
  categoryName?: InputMaybe<Scalars['String']['input']>;
  endTime: Scalars['String']['input'];
  icon?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  importedAt: Scalars['String']['input'];
  isDisplayHomeView: Scalars['Boolean']['input'];
  isMiddleDay: Scalars['Boolean']['input'];
  isUndefinedEndedAt: Scalars['Boolean']['input'];
  largeCategory?: InputMaybe<Scalars['String']['input']>;
  memo?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  startTime: Scalars['String']['input'];
};

export type EventScheduleOrderInput = {
  allDay?: InputMaybe<OrderByEnum>;
  categoryName?: InputMaybe<OrderByEnum>;
  endTime?: InputMaybe<OrderByEnum>;
  icon?: InputMaybe<OrderByEnum>;
  id?: InputMaybe<OrderByEnum>;
  importedAt?: InputMaybe<OrderByEnum>;
  isDisplayHomeView?: InputMaybe<OrderByEnum>;
  isMiddleDay?: InputMaybe<OrderByEnum>;
  isUndefinedEndedAt?: InputMaybe<OrderByEnum>;
  largeCategory?: InputMaybe<OrderByEnum>;
  memo?: InputMaybe<OrderByEnum>;
  name?: InputMaybe<OrderByEnum>;
  rawData?: InputMaybe<OrderByEnum>;
  startTime?: InputMaybe<OrderByEnum>;
};

export type EventScheduleUpdateInput = {
  allDay?: InputMaybe<Scalars['Boolean']['input']>;
  categoryName?: InputMaybe<Scalars['String']['input']>;
  endTime?: InputMaybe<Scalars['String']['input']>;
  icon?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['String']['input']>;
  importedAt?: InputMaybe<Scalars['String']['input']>;
  isDisplayHomeView?: InputMaybe<Scalars['Boolean']['input']>;
  isMiddleDay?: InputMaybe<Scalars['Boolean']['input']>;
  isUndefinedEndedAt?: InputMaybe<Scalars['Boolean']['input']>;
  largeCategory?: InputMaybe<Scalars['String']['input']>;
  memo?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
  startTime?: InputMaybe<Scalars['String']['input']>;
};

export type IntegerFilterInput = {
  between?: InputMaybe<Array<Scalars['Int']['input']>>;
  eq?: InputMaybe<Scalars['Int']['input']>;
  gt?: InputMaybe<Scalars['Int']['input']>;
  gte?: InputMaybe<Scalars['Int']['input']>;
  is_in?: InputMaybe<Array<Scalars['Int']['input']>>;
  is_not_in?: InputMaybe<Array<Scalars['Int']['input']>>;
  is_not_null?: InputMaybe<Scalars['Int']['input']>;
  is_null?: InputMaybe<Scalars['Int']['input']>;
  lt?: InputMaybe<Scalars['Int']['input']>;
  lte?: InputMaybe<Scalars['Int']['input']>;
  ne?: InputMaybe<Scalars['Int']['input']>;
  not_between?: InputMaybe<Array<Scalars['Int']['input']>>;
};

export type Mutation = {
  __typename?: 'Mutation';
  _ping?: Maybe<Scalars['String']['output']>;
  calendarCreateBatch: Array<CalendarBasic>;
  calendarCreateOne: CalendarBasic;
  calendarDelete: Scalars['Int']['output'];
  calendarUpdate: Array<CalendarBasic>;
  contentItemCreateBatch: Array<ContentItemBasic>;
  contentItemCreateOne: ContentItemBasic;
  contentItemDelete: Scalars['Int']['output'];
  contentItemTagCreateBatch: Array<ContentItemTagBasic>;
  contentItemTagCreateOne: ContentItemTagBasic;
  contentItemTagDelete: Scalars['Int']['output'];
  contentItemTagUpdate: Array<ContentItemTagBasic>;
  contentItemUpdate: Array<ContentItemBasic>;
  eventScheduleCreateBatch: Array<EventScheduleBasic>;
  eventScheduleCreateOne: EventScheduleBasic;
  eventScheduleDelete: Scalars['Int']['output'];
  eventScheduleUpdate: Array<EventScheduleBasic>;
  tagCreateBatch: Array<TagBasic>;
  tagCreateOne: TagBasic;
  tagDelete: Scalars['Int']['output'];
  tagUpdate: Array<TagBasic>;
};


export type MutationCalendarCreateBatchArgs = {
  data: Array<CalendarInsertInput>;
};


export type MutationCalendarCreateOneArgs = {
  data: CalendarInsertInput;
};


export type MutationCalendarDeleteArgs = {
  filter?: InputMaybe<CalendarFilterInput>;
};


export type MutationCalendarUpdateArgs = {
  data: CalendarUpdateInput;
  filter?: InputMaybe<CalendarFilterInput>;
};


export type MutationContentItemCreateBatchArgs = {
  data: Array<ContentItemInsertInput>;
};


export type MutationContentItemCreateOneArgs = {
  data: ContentItemInsertInput;
};


export type MutationContentItemDeleteArgs = {
  filter?: InputMaybe<ContentItemFilterInput>;
};


export type MutationContentItemTagCreateBatchArgs = {
  data: Array<ContentItemTagInsertInput>;
};


export type MutationContentItemTagCreateOneArgs = {
  data: ContentItemTagInsertInput;
};


export type MutationContentItemTagDeleteArgs = {
  filter?: InputMaybe<ContentItemTagFilterInput>;
};


export type MutationContentItemTagUpdateArgs = {
  data: ContentItemTagUpdateInput;
  filter?: InputMaybe<ContentItemTagFilterInput>;
};


export type MutationContentItemUpdateArgs = {
  data: ContentItemUpdateInput;
  filter?: InputMaybe<ContentItemFilterInput>;
};


export type MutationEventScheduleCreateBatchArgs = {
  data: Array<EventScheduleInsertInput>;
};


export type MutationEventScheduleCreateOneArgs = {
  data: EventScheduleInsertInput;
};


export type MutationEventScheduleDeleteArgs = {
  filter?: InputMaybe<EventScheduleFilterInput>;
};


export type MutationEventScheduleUpdateArgs = {
  data: EventScheduleUpdateInput;
  filter?: InputMaybe<EventScheduleFilterInput>;
};


export type MutationTagCreateBatchArgs = {
  data: Array<TagInsertInput>;
};


export type MutationTagCreateOneArgs = {
  data: TagInsertInput;
};


export type MutationTagDeleteArgs = {
  filter?: InputMaybe<TagFilterInput>;
};


export type MutationTagUpdateArgs = {
  data: TagUpdateInput;
  filter?: InputMaybe<TagFilterInput>;
};

export type OffsetInput = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
};

export enum OrderByEnum {
  Asc = 'ASC',
  Desc = 'DESC'
}

export type PageInfo = {
  __typename?: 'PageInfo';
  endCursor?: Maybe<Scalars['String']['output']>;
  hasNextPage: Scalars['Boolean']['output'];
  hasPreviousPage: Scalars['Boolean']['output'];
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type PageInput = {
  limit: Scalars['Int']['input'];
  page: Scalars['Int']['input'];
};

export type PaginationInfo = {
  __typename?: 'PaginationInfo';
  current: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  pages: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type PaginationInput = {
  cursor?: InputMaybe<CursorInput>;
  offset?: InputMaybe<OffsetInput>;
  page?: InputMaybe<PageInput>;
};

export type Query = {
  __typename?: 'Query';
  _sea_orm_entity_metadata?: Maybe<Scalars['String']['output']>;
  calendar: CalendarConnection;
  contentItem: ContentItemConnection;
  contentItemTag: ContentItemTagConnection;
  eventSchedule: EventScheduleConnection;
  tag: TagConnection;
};


export type Query_Sea_Orm_Entity_MetadataArgs = {
  table_name: Scalars['String']['input'];
};


export type QueryCalendarArgs = {
  filters?: InputMaybe<CalendarFilterInput>;
  orderBy?: InputMaybe<CalendarOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};


export type QueryContentItemArgs = {
  filters?: InputMaybe<ContentItemFilterInput>;
  orderBy?: InputMaybe<ContentItemOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};


export type QueryContentItemTagArgs = {
  filters?: InputMaybe<ContentItemTagFilterInput>;
  orderBy?: InputMaybe<ContentItemTagOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};


export type QueryEventScheduleArgs = {
  filters?: InputMaybe<EventScheduleFilterInput>;
  orderBy?: InputMaybe<EventScheduleOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};


export type QueryTagArgs = {
  filters?: InputMaybe<TagFilterInput>;
  orderBy?: InputMaybe<TagOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};

export enum RepeatTypeEnum {
  None = 'none',
  Yearly = 'yearly'
}

export type RepeatTypeEnumFilterInput = {
  eq?: InputMaybe<RepeatTypeEnum>;
  gt?: InputMaybe<RepeatTypeEnum>;
  gte?: InputMaybe<RepeatTypeEnum>;
  is_in?: InputMaybe<Array<RepeatTypeEnum>>;
  is_not_in?: InputMaybe<Array<RepeatTypeEnum>>;
  is_not_null?: InputMaybe<RepeatTypeEnum>;
  is_null?: InputMaybe<RepeatTypeEnum>;
  lt?: InputMaybe<RepeatTypeEnum>;
  lte?: InputMaybe<RepeatTypeEnum>;
  ne?: InputMaybe<RepeatTypeEnum>;
};

export enum SkinIpEnum {
  AnimalCrossing = 'animal_crossing',
  Mario = 'mario',
  Pikmin = 'pikmin',
  Splatoon = 'splatoon',
  Zelda = 'zelda'
}

export type SkinIpEnumFilterInput = {
  eq?: InputMaybe<SkinIpEnum>;
  gt?: InputMaybe<SkinIpEnum>;
  gte?: InputMaybe<SkinIpEnum>;
  is_in?: InputMaybe<Array<SkinIpEnum>>;
  is_not_in?: InputMaybe<Array<SkinIpEnum>>;
  is_not_null?: InputMaybe<SkinIpEnum>;
  is_null?: InputMaybe<SkinIpEnum>;
  lt?: InputMaybe<SkinIpEnum>;
  lte?: InputMaybe<SkinIpEnum>;
  ne?: InputMaybe<SkinIpEnum>;
};

export type StringFilterInput = {
  between?: InputMaybe<Array<Scalars['String']['input']>>;
  contains?: InputMaybe<Scalars['String']['input']>;
  ends_with?: InputMaybe<Scalars['String']['input']>;
  eq?: InputMaybe<Scalars['String']['input']>;
  gt?: InputMaybe<Scalars['String']['input']>;
  gte?: InputMaybe<Scalars['String']['input']>;
  is_in?: InputMaybe<Array<Scalars['String']['input']>>;
  is_not_in?: InputMaybe<Array<Scalars['String']['input']>>;
  is_not_null?: InputMaybe<Scalars['String']['input']>;
  is_null?: InputMaybe<Scalars['String']['input']>;
  like?: InputMaybe<Scalars['String']['input']>;
  lt?: InputMaybe<Scalars['String']['input']>;
  lte?: InputMaybe<Scalars['String']['input']>;
  ne?: InputMaybe<Scalars['String']['input']>;
  not_between?: InputMaybe<Array<Scalars['String']['input']>>;
  not_like?: InputMaybe<Scalars['String']['input']>;
  starts_with?: InputMaybe<Scalars['String']['input']>;
};

export type Tag = {
  __typename?: 'Tag';
  contentItemTag: ContentItemTagConnection;
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  name: Scalars['String']['output'];
  platform?: Maybe<Array<Scalars['String']['output']>>;
  platformText?: Maybe<Scalars['String']['output']>;
  searchHeaderImage?: Maybe<Scalars['String']['output']>;
  searchViewTagColor?: Maybe<Scalars['String']['output']>;
};


export type TagContentItemTagArgs = {
  filters?: InputMaybe<ContentItemTagFilterInput>;
  orderBy?: InputMaybe<ContentItemTagOrderInput>;
  pagination?: InputMaybe<PaginationInput>;
};

export type TagBasic = {
  __typename?: 'TagBasic';
  icon?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  importedAt: Scalars['String']['output'];
  name: Scalars['String']['output'];
  platform?: Maybe<Array<Scalars['String']['output']>>;
  platformText?: Maybe<Scalars['String']['output']>;
  searchHeaderImage?: Maybe<Scalars['String']['output']>;
  searchViewTagColor?: Maybe<Scalars['String']['output']>;
};

export type TagConnection = {
  __typename?: 'TagConnection';
  edges: Array<TagEdge>;
  nodes: Array<Tag>;
  pageInfo: PageInfo;
  paginationInfo?: Maybe<PaginationInfo>;
};

export type TagEdge = {
  __typename?: 'TagEdge';
  cursor: Scalars['String']['output'];
  node: Tag;
};

export type TagFilterInput = {
  and?: InputMaybe<Array<TagFilterInput>>;
  icon?: InputMaybe<StringFilterInput>;
  id?: InputMaybe<StringFilterInput>;
  importedAt?: InputMaybe<TextFilterInput>;
  name?: InputMaybe<StringFilterInput>;
  or?: InputMaybe<Array<TagFilterInput>>;
  platformText?: InputMaybe<StringFilterInput>;
  searchHeaderImage?: InputMaybe<StringFilterInput>;
  searchViewTagColor?: InputMaybe<StringFilterInput>;
};

export type TagInsertInput = {
  icon?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  importedAt: Scalars['String']['input'];
  name: Scalars['String']['input'];
  platform?: InputMaybe<Array<Scalars['String']['input']>>;
  platformText?: InputMaybe<Scalars['String']['input']>;
  searchHeaderImage?: InputMaybe<Scalars['String']['input']>;
  searchViewTagColor?: InputMaybe<Scalars['String']['input']>;
};

export type TagOrderInput = {
  icon?: InputMaybe<OrderByEnum>;
  id?: InputMaybe<OrderByEnum>;
  importedAt?: InputMaybe<OrderByEnum>;
  name?: InputMaybe<OrderByEnum>;
  platform?: InputMaybe<OrderByEnum>;
  platformText?: InputMaybe<OrderByEnum>;
  rawData?: InputMaybe<OrderByEnum>;
  searchHeaderImage?: InputMaybe<OrderByEnum>;
  searchViewTagColor?: InputMaybe<OrderByEnum>;
};

export type TagUpdateInput = {
  icon?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['String']['input']>;
  importedAt?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
  platform?: InputMaybe<Array<Scalars['String']['input']>>;
  platformText?: InputMaybe<Scalars['String']['input']>;
  searchHeaderImage?: InputMaybe<Scalars['String']['input']>;
  searchViewTagColor?: InputMaybe<Scalars['String']['input']>;
};

export type TextFilterInput = {
  between?: InputMaybe<Array<Scalars['String']['input']>>;
  eq?: InputMaybe<Scalars['String']['input']>;
  gt?: InputMaybe<Scalars['String']['input']>;
  gte?: InputMaybe<Scalars['String']['input']>;
  is_in?: InputMaybe<Array<Scalars['String']['input']>>;
  is_not_in?: InputMaybe<Array<Scalars['String']['input']>>;
  is_not_null?: InputMaybe<Scalars['String']['input']>;
  is_null?: InputMaybe<Scalars['String']['input']>;
  lt?: InputMaybe<Scalars['String']['input']>;
  lte?: InputMaybe<Scalars['String']['input']>;
  ne?: InputMaybe<Scalars['String']['input']>;
  not_between?: InputMaybe<Array<Scalars['String']['input']>>;
};

export enum TitleColorEnum {
  Black = 'black',
  White = 'white'
}

export type TitleColorEnumFilterInput = {
  eq?: InputMaybe<TitleColorEnum>;
  gt?: InputMaybe<TitleColorEnum>;
  gte?: InputMaybe<TitleColorEnum>;
  is_in?: InputMaybe<Array<TitleColorEnum>>;
  is_not_in?: InputMaybe<Array<TitleColorEnum>>;
  is_not_null?: InputMaybe<TitleColorEnum>;
  is_null?: InputMaybe<TitleColorEnum>;
  lt?: InputMaybe<TitleColorEnum>;
  lte?: InputMaybe<TitleColorEnum>;
  ne?: InputMaybe<TitleColorEnum>;
};

export type GetCalendarDataQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCalendarDataQuery = { __typename?: 'Query', calendar: { __typename?: 'CalendarConnection', nodes: Array<{ __typename?: 'Calendar', id: string, day: number, month: number, year: number, animation?: string | null, thumbnail?: string | null, animationType?: AnimationTypeEnum | null }> } };

export type GetContentQueryVariables = Exact<{
  id: Scalars['String']['input'];
}>;


export type GetContentQuery = { __typename?: 'Query', contentItem: { __typename?: 'ContentItemConnection', nodes: Array<{ __typename?: 'ContentItem', id: string, title: string, contentBodyUrl?: string | null, note?: string | null, contentMovie?: string | null, contentImages?: Array<string> | null }> } };

export type GetAllContentsQueryVariables = Exact<{
  page: Scalars['Int']['input'];
}>;


export type GetAllContentsQuery = { __typename?: 'Query', contentItem: { __typename?: 'ContentItemConnection', nodes: Array<{ __typename?: 'ContentItem', id: string, thumbnail?: string | null, title: string, importedAt: string }>, paginationInfo?: { __typename?: 'PaginationInfo', pages: number, total: number } | null } };

export type GetEventDataQueryVariables = Exact<{
  page: Scalars['Int']['input'];
}>;


export type GetEventDataQuery = { __typename?: 'Query', eventSchedule: { __typename?: 'EventScheduleConnection', nodes: Array<{ __typename?: 'EventSchedule', id: string, name: string, memo?: string | null, categoryName?: string | null, largeCategory?: string | null, startTime: string, endTime: string, icon?: string | null, importedAt: string }>, paginationInfo?: { __typename?: 'PaginationInfo', pages: number, total: number } | null } };

export type GetTagContentsDataQueryVariables = Exact<{
  page: Scalars['Int']['input'];
  tag?: InputMaybe<Scalars['String']['input']>;
}>;


export type GetTagContentsDataQuery = { __typename?: 'Query', tag: { __typename?: 'TagConnection', nodes: Array<{ __typename?: 'Tag', id: string, name: string, icon?: string | null, searchHeaderImage?: string | null, contentItemTag: { __typename?: 'ContentItemTagConnection', nodes: Array<{ __typename?: 'ContentItemTag', contentItem?: { __typename?: 'ContentItem', id: string, thumbnail?: string | null, title: string } | null }>, paginationInfo?: { __typename?: 'PaginationInfo', pages: number, total: number } | null } }> } };

export type GetTagDataQueryVariables = Exact<{
  page: Scalars['Int']['input'];
}>;


export type GetTagDataQuery = { __typename?: 'Query', tag: { __typename?: 'TagConnection', nodes: Array<{ __typename?: 'Tag', id: string, name: string, icon?: string | null, platformText?: string | null, importedAt: string }>, paginationInfo?: { __typename?: 'PaginationInfo', pages: number, total: number } | null } };


export const GetCalendarDataDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getCalendarData"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"calendar"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"1000"}},{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"IntValue","value":"0"}}]}}]}},{"kind":"Argument","name":{"kind":"Name","value":"orderBy"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"year"},"value":{"kind":"EnumValue","value":"ASC"}},{"kind":"ObjectField","name":{"kind":"Name","value":"month"},"value":{"kind":"EnumValue","value":"ASC"}},{"kind":"ObjectField","name":{"kind":"Name","value":"day"},"value":{"kind":"EnumValue","value":"ASC"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"day"}},{"kind":"Field","name":{"kind":"Name","value":"month"}},{"kind":"Field","name":{"kind":"Name","value":"year"}},{"kind":"Field","name":{"kind":"Name","value":"animation"}},{"kind":"Field","name":{"kind":"Name","value":"thumbnail"}},{"kind":"Field","name":{"kind":"Name","value":"animationType"}}]}}]}}]}}]} as unknown as DocumentNode<GetCalendarDataQuery, GetCalendarDataQueryVariables>;
export const GetContentDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getContent"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"contentItem"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"filters"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"eq"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}]}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"contentBodyUrl"}},{"kind":"Field","name":{"kind":"Name","value":"note"}},{"kind":"Field","name":{"kind":"Name","value":"contentMovie"}},{"kind":"Field","name":{"kind":"Name","value":"contentImages"}}]}}]}}]}}]} as unknown as DocumentNode<GetContentQuery, GetContentQueryVariables>;
export const GetAllContentsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getAllContents"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"contentItem"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"8"}},{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}]}}]}},{"kind":"Argument","name":{"kind":"Name","value":"orderBy"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"importedAt"},"value":{"kind":"EnumValue","value":"ASC"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"thumbnail"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"importedAt"}}]}},{"kind":"Field","name":{"kind":"Name","value":"paginationInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pages"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<GetAllContentsQuery, GetAllContentsQueryVariables>;
export const GetEventDataDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getEventData"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"eventSchedule"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"7"}},{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}]}}]}},{"kind":"Argument","name":{"kind":"Name","value":"orderBy"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"startTime"},"value":{"kind":"EnumValue","value":"ASC"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"memo"}},{"kind":"Field","name":{"kind":"Name","value":"categoryName"}},{"kind":"Field","name":{"kind":"Name","value":"largeCategory"}},{"kind":"Field","name":{"kind":"Name","value":"startTime"}},{"kind":"Field","name":{"kind":"Name","value":"endTime"}},{"kind":"Field","name":{"kind":"Name","value":"icon"}},{"kind":"Field","name":{"kind":"Name","value":"importedAt"}}]}},{"kind":"Field","name":{"kind":"Name","value":"paginationInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pages"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<GetEventDataQuery, GetEventDataQueryVariables>;
export const GetTagContentsDataDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getTagContentsData"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"tag"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tag"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"filters"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"eq"},"value":{"kind":"Variable","name":{"kind":"Name","value":"tag"}}}]}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"icon"}},{"kind":"Field","name":{"kind":"Name","value":"searchHeaderImage"}},{"kind":"Field","name":{"kind":"Name","value":"contentItemTag"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"8"}},{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}]}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"contentItem"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"thumbnail"}},{"kind":"Field","name":{"kind":"Name","value":"title"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"paginationInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pages"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<GetTagContentsDataQuery, GetTagContentsDataQueryVariables>;
export const GetTagDataDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getTagData"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tag"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"8"}},{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}]}}]}},{"kind":"Argument","name":{"kind":"Name","value":"orderBy"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"EnumValue","value":"ASC"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"icon"}},{"kind":"Field","name":{"kind":"Name","value":"platformText"}},{"kind":"Field","name":{"kind":"Name","value":"importedAt"}}]}},{"kind":"Field","name":{"kind":"Name","value":"paginationInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pages"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<GetTagDataQuery, GetTagDataQueryVariables>;