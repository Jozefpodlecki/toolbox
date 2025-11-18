<script lang="ts">
    import type { Paged, Program } from "$lib/types";
    import IconChevronRight from "~icons/tabler/chevron-right";
    import IconChevronRightPipe from "~icons/tabler/chevron-right-pipe";
    import IconChevronLeft from "~icons/tabler/chevron-left";
    import IconChevronLeftPipe from "~icons/tabler/chevron-left-pipe";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        page: number;
        pageSize: number;
        total: number;
    }

    let { page = $bindable(), pageSize = $bindable(), total }: Props = $props();

    const onPage: MouseEventHandler<HTMLButtonElement> = (event) => {
        const value = Number(event.currentTarget.dataset.value);
        page = Math.min(Math.max(value, firstPage), lastPage);
    }

    const pageCount = $derived(Math.ceil(total / pageSize));
    const firstPage = $derived(0);
    const lastPage = $derived(pageCount > 0 ? pageCount - 1 : 0);

</script>


<footer class="flex items-center gap-2 text-sm">
    <select bind:value={pageSize} class="bg-neutral-900 text-xs text-neutral-200 border-0">
        <option value={10}>10</option>
        <option value={25}>25</option>
        <option value={50}>50</option>
    </select>
    <button data-value={firstPage} disabled={page === firstPage} type="button" onclick={onPage} class=""><IconChevronLeftPipe/></button>
    <button data-value={page - 1} disabled={page === firstPage} type="button" onclick={onPage} class=""><IconChevronLeft/></button>
    <span>{page + 1} / {pageCount}</span>
    <button data-value={page + 1} disabled={page === lastPage} type="button" onclick={onPage} class=""><IconChevronRight/></button>
    <button data-value={lastPage} disabled={page === lastPage} type="button" onclick={onPage} class=""><IconChevronRightPipe/></button>
</footer>