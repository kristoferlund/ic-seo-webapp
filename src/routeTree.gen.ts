/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as Index2Import } from './routes/index2'
import { Route as IndexImport } from './routes/index'
import { Route as SubpathIdImport } from './routes/subpath/$id'

// Create/Update Routes

const Index2Route = Index2Import.update({
  id: '/index2',
  path: '/index2',
  getParentRoute: () => rootRoute,
} as any)

const IndexRoute = IndexImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const SubpathIdRoute = SubpathIdImport.update({
  id: '/subpath/$id',
  path: '/subpath/$id',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/index2': {
      id: '/index2'
      path: '/index2'
      fullPath: '/index2'
      preLoaderRoute: typeof Index2Import
      parentRoute: typeof rootRoute
    }
    '/subpath/$id': {
      id: '/subpath/$id'
      path: '/subpath/$id'
      fullPath: '/subpath/$id'
      preLoaderRoute: typeof SubpathIdImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexRoute
  '/index2': typeof Index2Route
  '/subpath/$id': typeof SubpathIdRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRoute
  '/index2': typeof Index2Route
  '/subpath/$id': typeof SubpathIdRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRoute
  '/index2': typeof Index2Route
  '/subpath/$id': typeof SubpathIdRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths: '/' | '/index2' | '/subpath/$id'
  fileRoutesByTo: FileRoutesByTo
  to: '/' | '/index2' | '/subpath/$id'
  id: '__root__' | '/' | '/index2' | '/subpath/$id'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRoute: typeof IndexRoute
  Index2Route: typeof Index2Route
  SubpathIdRoute: typeof SubpathIdRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRoute: IndexRoute,
  Index2Route: Index2Route,
  SubpathIdRoute: SubpathIdRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/index2",
        "/subpath/$id"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/index2": {
      "filePath": "index2.tsx"
    },
    "/subpath/$id": {
      "filePath": "subpath/$id.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
