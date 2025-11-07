<script lang="ts">
    import { page } from "$app/state";
    
    const id = page.params.id || "";

</script>

<div>{id}</div>