#ifndef ZEROSPACE_HELPER_CONFIG_H
#define ZEROSPACE_HELPER_CONFIG_H

/* ZeroSpace derivative identity. Keep upstream provenance in NOTICE.md and
 * preserve GPL notices when GPL-covered payload sources are imported. */
#define ZEROSPACE_HELPER_NAME "ZeroSpace Helper"
#define ZEROSPACE_HELPER_VERSION "0.1.0"
#define ZEROSPACE_HELPER_AUTHOR "Zer0Spce"
#define ZEROSPACE_HELPER_UPSTREAM "ps5upload by PhantomPtr"

/* FTX2-compatible ports used by the desktop integration. */
#define ZEROSPACE_TRANSFER_PORT 9113
#define ZEROSPACE_MGMT_PORT 9114

/* Keep ZeroSpace state separate from an installed ps5upload payload. */
#define ZEROSPACE_RUNTIME_ROOT "/data/zerospace"
#define ZEROSPACE_RUNTIME_DIR "/data/zerospace/runtime"
#define ZEROSPACE_DEBUG_DIR "/data/zerospace/debug"

#endif
