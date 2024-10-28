import type Profile from '~/models/profile'

export default function () {
  const selectedProfile = useState<Profile | null>('selectedProfile', () => null)

  const setSelectedProfile = (profile: Profile) => {
    selectedProfile.value = profile
  }

  return { profile: selectedProfile, setSelectedProfile }
}
