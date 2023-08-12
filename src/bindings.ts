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

export function updatePerson(person: Person) {
    return invoke()<Person>("update_person", { person })
}

export function getPlannedEvents() {
    return invoke()<MinistryEvent[]>("get_planned_events")
}

export function createEvent(newEvent: NewMinistryEvent) {
    return invoke()<MinistryEvent>("create_event", { newEvent })
}

export function updateEvent(event: MinistryEvent) {
    return invoke()<MinistryEvent>("update_event", { event })
}

export function deleteEvent(id: number) {
    return invoke()<null>("delete_event", { id })
}

export type NewPerson = { name: string }
export type Person = { id: number; name: string }
export type NewMinistryEvent = { assigneeName: string; assigneeId: number | null; date: string; time: string | null; place: string; extraInfo: string }
export type MinistryEvent = { id: number; assigneeName: string; assigneeId: number | null; date: string; time: string | null; place: string; extraInfo: string }
