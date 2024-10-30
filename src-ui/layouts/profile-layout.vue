<script lang="ts" setup>
import type { DropdownItem } from '#ui/types'

const { $profileService, $importExportService } = useNuxtApp()

const { profile } = storeToRefs(selectedProfile())
const { data: profiles } = useAsyncData('profiles', () => $profileService.getAll())

const filteredProfiles = computed(() =>
  (profiles.value || []).sort((a, b) => a.name.localeCompare(b.name))
)

const items = computed<DropdownItem[][]>(() => [
  [
    {
      label: 'Sélection du profil',
      disabled: true,
      icon: 'i-lucide-square-mouse-pointer',
      iconClass: 'size-5',
    },
  ],
  [
    ...filteredProfiles.value.map((p) => ({
      label: p.name,
      disabled: p.id === profile.value.id,
      href: `/profiles/${p.id}`,
      icon: p.id === profile.value.id ? 'i-lucide-circle-check-big' : 'i-lucide-circle',
    })),
    {
      label: 'Nouveau profil',
      icon: 'i-lucide-plus',
      href: '/profiles/create',
    },
  ],
  [
    {
      label: 'Réglages',
      disabled: true,
      icon: 'i-lucide-settings-2',
      iconClass: 'size-5',
    },
  ],
  [
    {
      label: 'Configuration du profil',
      href: `/profiles/${profile.value.id}/settings`,
      icon: 'i-lucide-file-cog',
    },
    {
      label: 'Importer une sauvegarde',
      icon: 'i-lucide-download',
      click: () => {
        importConfirmationModalOpen.value = true
      },
    },
    {
      label: 'Exporter les données',
      icon: 'i-lucide-upload',
      click: handleExport,
    },
  ],
  [
    {
      label: 'Quitter',
      icon: 'i-lucide-log-out',
      href: '/',
    },
  ],
])

const toast = useToast()
const importConfirmationModalOpen = ref(false)

async function handleImport() {
  try {
    await $importExportService.import()
    toast.add({ id: 'import_success', title: 'Données importées avec succès !', color: 'green' })
    navigateTo('/')
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'import_fail', title: error })
    } else {
      toast.add({ id: 'import_fail', title: 'Une erreur est survenue' })
    }
  }
}

async function handleExport() {
  try {
    await $importExportService.export()
    toast.add({ id: 'export_success', title: 'Données exportées avec succès !', color: 'green' })
  } catch (error) {
    if (typeof error === 'string') {
      toast.add({ id: 'export_fail', title: error })
    } else {
      toast.add({ id: 'export_fail', title: 'Une erreur est survenue' })
    }
  }
}

const links = computed(
  () =>
    [
      {
        label: 'Accueil',
        to: `/profiles/${profile.value.id}`,
      },
      {
        label: 'Véhicules',
        to: `/profiles/${profile.value.id}/vehicles`,
      },
    ] as const
)
</script>

<template>
  <div class="min-h-screen">
    <header class="h-16 p-2 border-b border-neutral-600 grid grid-cols-3 items-center">
      <UDropdown
        mode="click"
        :items="items"
        :popper="{ arrow: true, placement: 'bottom-start' }"
        :ui="{ width: 'w-56', item: { icon: { base: 'size-3' } } }"
      >
        <UButton
          color="white"
          size="lg"
          class="w-56 justify-between"
        >
          <p class="text-base truncate">{{ profile!.name }}</p>
          <IChevronDown class="size-6 shrink-0" />
        </UButton>
      </UDropdown>

      <nav class="flex items-center justify-center gap-x-4">
        <!--suppress HtmlUnknownTarget -->
        <ULink
          :to="link.to"
          class="underline-offset-4"
          active-class="text-primary underline"
          inactive-class="hover:underline"
          v-for="(link, index) of links"
          :key="index"
        >
          {{ link.label }}
        </ULink>
      </nav>
    </header>
    <slot></slot>

    <UModal v-model="importConfirmationModalOpen">
      <UCard>
        <div class="space-y-4">
          <h3 class="font-semibold text-xl text-center text-balance">
            Vérifiez que vous avez bien exporté vos données actuelles !
          </h3>

          <p class="text-neutral-400 text-sm font-thin flex items-center">
            <IAlertTriangle class="size-4 mr-2" />
            Les données non exportées seront écrasées
          </p>

          <div class="grid grid-cols-2 gap-4">
            <UButton
              variant="soft"
              @click="importConfirmationModalOpen = false"
              block
            >
              Annuler
            </UButton>
            <UButton
              @click="
                () => {
                  importConfirmationModalOpen = false
                  handleImport()
                }
              "
              block
            >
              Poursuivre
            </UButton>
          </div>
        </div>
      </UCard>
    </UModal>
  </div>
</template>
