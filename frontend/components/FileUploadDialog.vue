<template>
    <dialog ref="uploadDialog" class="p-8 rounded-lg shadow-xl bg-white w-full max-w-2xl">
        <h2 class="text-2xl font-bold mb-6">Upload <code class="text-violet-500">package.json</code> of project</h2>
        <div class="mb-6 border-2 border-dashed border-gray-300 rounded-lg p-6 text-center cursor-pointer hover:border-violet-500 transition-colors"
            @click="triggerFileInput" @dragover.prevent @drop.prevent="handleFileDrop">
            <input type="file" accept=".json" @change="handleFileChange" ref="fileInput" class="hidden">
            <p class="text-lg text-gray-500 mb-2">
                Drag the file here or click to upload from computer
            </p>
            <p v-if="selectedFile" class="text-violet-600 font-semibold">
                Selected: {{ selectedFile.name }}
            </p>
            <p v-else class="text-sm text-gray-400">
                Supported format: JSON
            </p>
        </div>
        <p v-if="errorMessage" class="text-red-500 mb-4">{{ errorMessage }}</p>
        <div class="flex justify-end space-x-4">
            <button @click="closeDialog"
                class="px-6 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors">
                Cancel
            </button>
            <ButtonPrimary :disabled="!isFileValid || !fileInput" @click="handleScan">
                Scan
            </ButtonPrimary>
        </div>
    </dialog>
</template>

<script setup>
const uploadDialog = ref(null)
const fileInput = ref(null)
const selectedFile = ref(null)
const isFileValid = ref(false)
const errorMessage = ref('')

const emit = defineEmits(['file-scanned'])

function openDialog() {
    uploadDialog.value.showModal()
}

function closeDialog() {
    uploadDialog.value.close()
    resetForm()
}

function triggerFileInput() {
    fileInput.value.click()
}

function handleFileChange(event) {
    const file = event.target.files[0]
    validateAndSetFile(file)
}

function handleFileDrop(event) {
    const file = event.dataTransfer.files[0]
    validateAndSetFile(file)
}

function validateAndSetFile(file) {
    selectedFile.value = file
    if (file && file.type === 'application/json') {
        isFileValid.value = true
        errorMessage.value = ''
    } else {
        isFileValid.value = false
        errorMessage.value = 'Please select a valid JSON file.'
    }
}

function handleScan() {
    if (isFileValid.value) {
        const formData = new FormData()
        formData.append('file', selectedFile.value)
        emit('file-scanned', formData)
        closeDialog()
    }
}

function resetForm() {
    selectedFile.value = null
    isFileValid.value = false
    errorMessage.value = ''
    if (fileInput.value) {
        fileInput.value.value = ''
    }
}

defineExpose({ openDialog })
</script>
