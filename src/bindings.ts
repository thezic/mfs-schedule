/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function createPerson(newPerson: NewPerson) {
    return invoke()<Person>("create_person", { newPerson })
}

export function deletePerson(id: number) {
    return invoke()<null>("delete_person", { id })
}

export function getPersonById(id: number) {
    return invoke()<Person>("get_person_by_id", { id })
}

export function getPersons() {
    return invoke()<Person[]>("get_persons")
}

export type NewPerson = { name: string }
export type Person = { id: number; name: string }
