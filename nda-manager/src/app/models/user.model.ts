export enum UserType {
    client = 'client',
    supplier = 'supplier'
}

// Tipos para o novo sistema de roles
export type UserRole = 'client' | 'supplier';
export type UserRoles = UserRole[];

export interface User {
    id: string;
    username: string;
    name: string;
    stellar_public_key: string;
    roles: UserRoles; // ["client"], ["supplier"], ou ["client", "supplier"]
    created_at: string; // ISO date string
}

export interface UserRegisterRequest {
    username: string;
    name: string;
    password: string;
    roles: UserRoles; // ["client"], ["supplier"], ou ["client", "supplier"]
}

export interface UserLoginRequest {
    username: string;
    password: string;
}

export interface AutoLoginRequest {
    user_name: string;
    user_id: string;
}

export interface UserResponse {
    id: string;
    username: string;
    name: string;
    stellar_public_key: string;
    roles: UserRoles; // ["client"], ["supplier"], ou ["client", "supplier"]
    created_at: string;
}

// Funções utilitárias para trabalhar com roles
export class UserUtils {
    /**
     * Verifica se o usuário tem um papel específico
     * @param user Usuário a ser verificado
     * @param role Papel a ser verificado
     * @returns true se o usuário tem o papel especificado
     */
    static hasRole(user: User | UserResponse, role: UserRole): boolean {
        return user.roles.includes(role);
    }

    /**
     * Verifica se o usuário pode atuar como cliente
     * @param user Usuário a ser verificado
     * @returns true se o usuário pode criar e gerenciar processos NDA
     */
    static isClient(user: User | UserResponse): boolean {
        return this.hasRole(user, 'client');
    }

    /**
     * Verifica se o usuário pode atuar como fornecedor
     * @param user Usuário a ser verificado
     * @returns true se o usuário pode acessar processos compartilhados
     */
    static isSupplier(user: User | UserResponse): boolean {
        return this.hasRole(user, 'supplier');
    }

    /**
     * Verifica se o usuário tem múltiplos papéis
     * @param user Usuário a ser verificado
     * @returns true se o usuário tem mais de um papel
     */
    static hasMultipleRoles(user: User | UserResponse): boolean {
        return user.roles.length > 1;
    }

    /**
     * Retorna uma descrição legível dos papéis do usuário
     * @param user Usuário a ser verificado
     * @returns Descrição dos papéis (ex: "Cliente", "Fornecedor", "Cliente e Fornecedor")
     */
    static getRoleDescription(user: User | UserResponse): string {
        const { roles } = user;
        
        if (roles.length === 0) {
            return 'Sem papel definido';
        }
        
        if (roles.length === 1) {
            return roles[0] === 'client' ? 'Cliente' : 'Fornecedor';
        }
        
        if (roles.includes('client') && roles.includes('supplier')) {
            return 'Cliente e Fornecedor';
        }
        
        // Para casos futuros com outros papéis
        return roles.map(role => role === 'client' ? 'Cliente' : 'Fornecedor').join(', ');
    }

    /**
     * Cria um array de roles a partir de valores individuais
     * @param roles Papéis individuais
     * @returns Array de roles válido
     */
    static createRoles(...roles: UserRole[]): UserRoles {
        // Remove duplicatas e garante ordem consistente
        const uniqueRoles = Array.from(new Set(roles));
        return uniqueRoles.sort(); // client vem antes de supplier alfabeticamente
    }
}
