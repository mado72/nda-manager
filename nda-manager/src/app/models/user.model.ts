export enum UserType {
    client = 'client',
    partner = 'partner'
}

// Types for the new roles system
export type UserRole = 'client' | 'partner';
export type UserRoles = UserRole[];

export interface User {
    id: string;
    username: string;
    name: string;
    stellar_public_key: string;
    roles: UserRoles; // ["client"], ["partner"], ou ["client", "partner"]
    created_at: string; // ISO date string
}

export interface UserRegisterRequest {
    username: string;
    name: string;
    password: string;
    roles: UserRoles; // ["client"], ["partner"], ou ["client", "partner"]
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
    roles: UserRoles; // ["client"], ["partner"], ou ["client", "partner"]
    created_at: string;
}

// Utility functions for working with roles
export class UserUtils {
    /**
     * Checks if the user has a specific role
     * @param user User to check
     * @param role Role to check
     * @returns true if the user has the specified role
     */
    static hasRole(user: User | UserResponse, role: UserRole): boolean {
        return user.roles.includes(role);
    }

    /**
     * Checks if the user can act as a client
     * @param user User to check
     * @returns true if the user can create and manage NDA processes
     */
    static isClient(user: User | UserResponse): boolean {
        return this.hasRole(user, 'client');
    }

    /**
     * Checks if the user can act as a partner
     * @param user User to check
     * @returns true if the user can access shared processes
     */
    static isPartner(user: User | UserResponse): boolean {
        return this.hasRole(user, 'partner');
    }

    /**
     * Checks if the user has multiple roles
     * @param user User to check
     * @returns true if the user has more than one role
     */
    static hasMultipleRoles(user: User | UserResponse): boolean {
        return user.roles.length > 1;
    }

    /**
     * Returns a readable description of the user's roles
     * @param user User to check
     * @returns Description of roles (e.g., "Client", "Partner", "Client and Partner")
     */
    static getRoleDescription(user: User | UserResponse): string {
        const { roles } = user;
        
        if (roles.length === 0) {
            return 'No role defined';
        }
        
        if (roles.length === 1) {
            return roles[0] === 'client' ? 'Client' : 'Partner';
        }
        
        if (roles.includes('client') && roles.includes('partner')) {
            return 'Client and Partner';
        }
        
        // For future cases with other roles
        return roles.map(role => role === 'client' ? 'Client' : 'Partner').join(', ');
    }

    /**
     * Creates a roles array from individual values
     * @param roles Individual roles
     * @returns Valid roles array
     */
    static createRoles(...roles: UserRole[]): UserRoles {
        // Remove duplicates and ensure consistent order
        const uniqueRoles = Array.from(new Set(roles));
        return uniqueRoles.sort(); // client comes before partner alphabetically
    }
}
