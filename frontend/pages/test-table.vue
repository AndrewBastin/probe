<template>
    <Header />
    <div class="container mx-auto p-4">
        <h1 class="text-3xl font-bold mb-6 text-violet-300">Scan Results</h1>
        <div class="border border-violet-600 rounded-lg overflow-hidden shadow-lg bg-gray-800 bg-opacity-50">
            <div v-for="(dep, _index) in dummyDependencies.objects" :key="dep.name"
                class="border-b border-violet-600 last:border-b-0">
                <div @click="toggleExpand(dep.name)"
                    class="flex text-violet-200 font-semibold justify-between bg-gray-800 bg-opacity-70 items-center p-5 cursor-pointer hover:bg-gray-700 transition-colors duration-200 ease-in-out shadow-sm">
                    <div class="font-medium text-lg">{{ cleanDependencyName(dep.name) }}</div>
                    <div class="flex justify-between items-baseline gap-6">
                        <div class="flex items-center gap-1">
                            <Icon name="mdi:license" class="text-violet-400" />
                            <span class="text-violet-300">{{ dep.license }}</span>
                        </div>
                        <div v-if="dep.github" class="flex items-center gap-2">
                            <NuxtLink :to="dep.github.repo_url" target="_blank"
                                class="text-violet-400 hover:text-violet-300 underline">
                                <Icon name="mdi:github"/>
                            </NuxtLink>
                        </div>
                    </div>
                </div>
                <div v-if="expandedDeps.includes(dep.name)"
                    class="p-5 bg-gray-800 bg-opacity-30 text-violet-100 shadow-inner"
                    :class="{ 'animate-fadeIn': expandedDeps.includes(dep.name) }">
                    <div v-if="dep.github" class="grid grid-cols-1 gap-4">
                        <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Description</h3>
                            {{ dep.github?.repo_desc || 'No description available' }}
                        </div>

                        <div class="grid grid-cols-1 gap-4 md:grid-cols-3 lg:grid-cols-3">
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Number of contributors</h3>
                                {{ dep.github?.no_of_contributors || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Open Issues</h3>
                                {{ dep.github?.open_issues || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Number of dependencies</h3>
                                {{ getDependencyData(dep.name).length }}
                            </div>
                        </div>
                        <div v-if="getDependencyData(dep.name).length > 0"
                            class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Dependencies</h3>
                            <ul class="list-square marker:text-violet-300 list-outside ml-4">
                                <li v-for="data in getDependencyData(dep.name).dependencies" :key="data" class="mb-2">
                                    <div class="inline-flex items-center">
                                        <NuxtLink v-if="getGitHubURL(data)" :to="getGitHubURL(data)" target="_blank"
                                            rel="noopener noreferrer"
                                            class="hover:text-violet-300 transition-colors duration-200 flex items-center">
                                            {{ cleanDependencyName(data) }}
                                            <Icon name="mdi:arrow-top-right" class="ml-1 w-4 h-4" />
                                        </NuxtLink>
                                        <span v-else>{{ cleanDependencyName(data) }}</span>
                                    </div>
                                </li>
                            </ul>
                        </div>
                        <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Funding Information</h3>
                            <div v-if="getFundingInfo(dep.name)">
                                <ul class="list-square marker:text-violet-300 list-outside ml-4">
                                    <li v-for="(links, platform) in getFundingInfo(dep.name)" :key="platform" class="mb-2">
                                        <div class="inline-flex items-center">
                                            <NuxtLink
                                                :to="links[0]"
                                                target="_blank" rel="noopener noreferrer"
                                                class="hover:text-violet-300 transition-colors duration-200 flex items-center">
                                                {{ formatPlatformName(platform) }}
                                                <Icon name="mdi:arrow-top-right" class="ml-1 w-4 h-4" />
                                            </NuxtLink>
                                        </div>
                                    </li>
                                </ul>
                            </div>
                            <p v-else class="text-violet-200">Unavailable</p>
                        </div>
                    </div>

                    <span v-else class="text-violet-200 text-sm italic">Off the GitHub Grid</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'

const expandedDeps = ref([])

const dummyDependencies = {
    "objects": [
        {
            "name": "zod@^3.22.4",
            "license": "MIT",
            "github": null
        },
        {
            "name": "@babel/code-frame@^7.0.0",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/babel/babel",
                "repo_desc": "🐠 Babel is a compiler for writing next generation JavaScript.",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 774,
                "avg_issue_closing_time_mins": 3827.6666666666665,
                "last_commit_at": "2024-07-27T12:58:22Z",
                "funding": null
            }
        },
        {
            "name": "js-tokens@^4.0.0",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/lydell/js-tokens",
                "repo_desc": "Tiny JavaScript tokenizer.",
                "no_of_contributors": 4,
                "total_issues": 30,
                "open_issues": 0,
                "avg_issue_closing_time_mins": 2047.1666666666667,
                "last_commit_at": "2024-06-19T12:26:29Z",
                "funding": null
            }
        },
        {
            "name": "lodash@^4.17.21",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/lodash/lodash",
                "repo_desc": "A modern JavaScript utility library delivering modularity, performance, & extras.",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 93,
                "avg_issue_closing_time_mins": 24875.466666666667,
                "last_commit_at": "2024-07-10T20:56:03Z",
                "funding": null
            }
        },
        {
            "name": "parser-ts@^0.7.0",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/gcanti/parser-ts",
                "repo_desc": "String parser combinators for TypeScript",
                "no_of_contributors": 9,
                "total_issues": 30,
                "open_issues": 7,
                "avg_issue_closing_time_mins": 68040.46666666666,
                "last_commit_at": "2023-04-26T14:37:51Z",
                "funding": null
            }
        },
        {
            "name": "chalk@^2.4.2",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/chalk/chalk",
                "repo_desc": "🖍 Terminal string styling done right",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 4,
                "avg_issue_closing_time_mins": 265.26666666666665,
                "last_commit_at": "2024-07-05T10:41:35Z",
                "funding": null
            }
        },
        {
            "name": "fp-ts@^2.16.1",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/gcanti/fp-ts",
                "repo_desc": "Functional programming in TypeScript",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 189,
                "avg_issue_closing_time_mins": 7689.6,
                "last_commit_at": "2024-07-25T15:36:16Z",
                "funding": {
                    "links": {
                        "github": [
                            "https://github.com/sponsors/gcanti"
                        ],
                        "open-collective": [
                            "https://github.com/sponsors/gcanti"
                        ]
                    }
                }
            }
        },
        {
            "name": "@babel/helper-validator-identifier@^7.24.7",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/babel/babel",
                "repo_desc": "🐠 Babel is a compiler for writing next generation JavaScript.",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 774,
                "avg_issue_closing_time_mins": 3827.6666666666665,
                "last_commit_at": "2024-07-27T12:58:22Z",
                "funding": null
            }
        },
        {
            "name": "picocolors@^1.0.0",
            "license": "ISC",
            "github": {
                "repo_url": "https://github.com/alexeyraspopov/picocolors",
                "repo_desc": "The tiniest and the fastest library for terminal output formatting with ANSI colors",
                "no_of_contributors": 9,
                "total_issues": 30,
                "open_issues": 23,
                "avg_issue_closing_time_mins": 79667.8,
                "last_commit_at": "2024-06-25T10:36:23Z",
                "funding": null
            }
        },
        {
            "name": "verzod@^0.2.2",
            "license": "MIT",
            "github": null
        },
        {
            "name": "@babel/highlight@^7.24.7",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/babel/babel",
                "repo_desc": "🐠 Babel is a compiler for writing next generation JavaScript.",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 774,
                "avg_issue_closing_time_mins": 3827.6666666666665,
                "last_commit_at": "2024-07-27T12:58:22Z",
                "funding": null
            }
        },
        {
            "name": "io-ts@^2.2.20",
            "license": "MIT",
            "github": {
                "repo_url": "https://github.com/gcanti/io-ts",
                "repo_desc": "Runtime type system for IO decoding/encoding",
                "no_of_contributors": 30,
                "total_issues": 30,
                "open_issues": 161,
                "avg_issue_closing_time_mins": 44309.86666666667,
                "last_commit_at": "2023-12-03T18:25:23Z",
                "funding": null
            }
        }
    ],
    "root_objects": [
        "fp-ts@^2.16.1",
        "io-ts@^2.2.20",
        "lodash@^4.17.21",
        "parser-ts@^0.7.0",
        "verzod@^0.2.2",
        "zod@^3.22.4"
    ],
    "depends_on": {
        "@babel/helper-validator-identifier@^7.24.7": [],
        "@babel/code-frame@^7.0.0": [
            "picocolors@^1.0.0",
            "@babel/highlight@^7.24.7",
            "picocolors@^1.0.0",
            "@babel/highlight@^7.24.7"
        ],
        "js-tokens@^4.0.0": [],
        "picocolors@^1.0.0": [],
        "parser-ts@^0.7.0": [
            "@babel/code-frame@^7.0.0",
            "@babel/code-frame@^7.0.0"
        ],
        "verzod@^0.2.2": [],
        "chalk@^2.4.2": [],
        "fp-ts@^2.16.1": [],
        "@babel/highlight@^7.24.7": [
            "js-tokens@^4.0.0",
            "@babel/helper-validator-identifier@^7.24.7",
            "chalk@^2.4.2",
            "picocolors@^1.0.0",
            "js-tokens@^4.0.0",
            "@babel/helper-validator-identifier@^7.24.7",
            "chalk@^2.4.2"
        ],
        "lodash@^4.17.21": [],
        "io-ts@^2.2.20": [],
        "zod@^3.22.4": []
    }
}


function toggleExpand(depName) {
    const index = expandedDeps.value.indexOf(depName)
    if (index > -1) {
        expandedDeps.value.splice(index, 1)
    } else {
        expandedDeps.value.push(depName)
    }
}

function getDependencyData(depName) {
    if (depName in dummyDependencies.depends_on) {
        const dependencies = dummyDependencies.depends_on[depName];
        const length = dependencies.length;

        if (length > 0) {
            return {
                length: length,
                dependencies: dependencies
            };
        } else {
            return {
                length: 0
            };
        }
    } else {
        return {
            length: 0
        };
    }
}

function cleanDependencyName(dependency) {
    // Split the string at the last '@'
    const lastAtIndex = dependency.lastIndexOf('@');

    if (lastAtIndex <= 0) {
        return dependency;
    }
    return dependency.substring(0, lastAtIndex);
}

function getGitHubURL(dependency) {
    return dummyDependencies.objects.find(dep => dep.name === dependency)?.github?.repo_url ?? null;
}

function getFundingInfo(dependency) {
    const fundingInfo = dummyDependencies.objects.find(dep => dep.name === dependency)?.github?.funding;
    return fundingInfo?.links || null;
}

function formatPlatformName(platform) {
  if (platform === 'github') return 'GitHub Sponsors';
  
  return platform
    .split(/[-_]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(' ');
}

</script>

<style>
@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.animate-fadeIn {
    animation: fadeIn 0.3s ease-out;
}
</style>